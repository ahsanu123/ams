namespace AMS.Extension;

using System.Data;
using System.Linq;
using System.Runtime.CompilerServices;
using AMS.Constant;
using AMS.InternalMigration;
using Dapper;
using Newtonsoft.Json;
using SqlKata;
using SqlKata.Compilers;

public class DapperSqlKataExtensionHelper
{
    public static void LogRaw(SqlResult sqlResult)
    {
        UtilityExtension.ShowCurrentPosition(sqlResult.Sql);
        UtilityExtension.ShowCurrentPosition(JsonConvert.SerializeObject(sqlResult.NamedBindings));
    }

    public static SqlResult CompileSqliteQuery(Query query)
    {
        var compiler = new SqliteCompiler();
        var sqlResult = compiler.Compile(query);

        return sqlResult;
    }

    public static SqlResult CompilePostgresqlQuery(Query query)
    {
        var compiler = new PostgresCompiler();
        var sqlResult = compiler.Compile(query);

        return sqlResult;
    }
}

public static class DapperSqlKataExtension
{
    public static async Task<IDbConnection> InsertToDatabase<T>(
        this IDbConnection conn,
        T value,
        bool removeId = false,
        Type TableName = null
    )
    {
        var keyPair = new List<KeyValuePair<string, object>>();
        foreach (var obj in value.GetType().GetProperties())
        {
            var excludedType = ModelMigrationList.listExcludedType.Any(exludedType =>
                obj.ToString().Contains(exludedType)
            );

            if ((obj.Name.ToLower() == "id" && removeId) || excludedType)
                continue;

            keyPair.Add(new KeyValuePair<string, object>(obj.Name, obj.GetValue(value)));
        }
        var className = TableName != null ? TableName.Name : typeof(T).Name;
        var InsertIntoDatabase_QUERY = new Query(className).AsInsert(keyPair);

        await conn.ExecuteSqlKataAsync(InsertIntoDatabase_QUERY, true);
        return conn;
    }

    public static async Task<IDbConnection> UpdateByCondition<T>(
        this IDbConnection conn,
        T value,
        Action<Query> whereClause,
        bool removeId = false,
        Type TableName = null
    )
    {
        var keyPair = new List<KeyValuePair<string, object>>();
        foreach (var obj in value.GetType().GetProperties())
        {
            if (obj.Name.ToLower() == "id" && removeId)
                continue;
            keyPair.Add(new KeyValuePair<string, object>(obj.Name, obj.GetValue(value)));
        }

        var updateQuery = new Query(TableName != null ? TableName.Name : nameof(T));

        whereClause(updateQuery);
        updateQuery.AsUpdate(keyPair);

        await conn.ExecuteSqlKataAsync(updateQuery);
        return conn;
    }

    /// <summary>
    ///  Add Id to Database Array
    ///  used model need to have primaryId, and newInsertedId
    /// </summary>
    /// <param name="conn"></param>
    /// <param name="value"></param>
    /// <typeparam name="T"></typeparam>
    /// <returns></returns>
    public static async Task AddIdToDatabaseArray<T>(IDbConnection conn, T value)
    {
        var className = typeof(T).Name;

        var DataBaseList_Query = new Query(className);
        var CheckIfInsertedValueAlredyExist_Query = DataBaseList_Query
            .Clone()
            .SelectAllClassProperties(new[] { typeof(T) })
            .Where(value);

        var InsertNewValueToDatabase_Query = DataBaseList_Query.Clone().AsInsert(value);

        var isInsertedValuAlreadyExist = await conn.QuerySingleSqlKataAsync<T>(
            CheckIfInsertedValueAlredyExist_Query,
            true
        );

        if (isInsertedValuAlreadyExist == null)
        {
            await conn.ExecuteSqlKataAsync(InsertNewValueToDatabase_Query, true);
        }
    }

    public static async Task<T> QuerySingleSqlKataAsync<T>(
        this IDbConnection conn,
        Query query,
        bool LogRaw = false
    )
    {
        var sqlResult = DapperSqlKataExtensionHelper.CompilePostgresqlQuery(query);

        if (LogRaw)
        {
            UtilityExtension.ShowCurrentPosition(sqlResult.Sql);
        }

        var result = await conn.QueryAsync<T>(sqlResult.Sql, sqlResult.NamedBindings);
        return result.ToList().FirstOrDefault();
    }

    public static async Task ExecuteSqlKataAsync(
        this IDbConnection conn,
        Query query,
        bool logRaw = false,
        SqlCompilerConstant compiler = SqlCompilerConstant.Sqlite
    )
    {
        SqlResult sqlResult = null;

        switch (compiler)
        {
            case SqlCompilerConstant.Postgresql:
                sqlResult = DapperSqlKataExtensionHelper.CompilePostgresqlQuery(query);
                break;
            case SqlCompilerConstant.Sqlite:
                sqlResult = DapperSqlKataExtensionHelper.CompileSqliteQuery(query);
                break;
        }

        DapperSqlKataExtensionHelper.LogRaw(sqlResult);

        await conn.ExecuteAsync(sqlResult.Sql, sqlResult.NamedBindings);
    }

    public static async Task<IEnumerable<T>> QuerySqlKataAsync<T>(
        this IDbConnection conn,
        Query query,
        bool LogRaw = false,
        SqlCompilerConstant compiler = SqlCompilerConstant.Sqlite
    )
    {
        SqlResult sqlResult = null;

        switch (compiler)
        {
            case SqlCompilerConstant.Postgresql:
                sqlResult = DapperSqlKataExtensionHelper.CompilePostgresqlQuery(query);
                break;
            case SqlCompilerConstant.Sqlite:
                sqlResult = DapperSqlKataExtensionHelper.CompileSqliteQuery(query);
                break;
        }

        if (compiler == null)
            throw new ArgumentNullException("Compiler not found!!");

        DapperSqlKataExtensionHelper.LogRaw(sqlResult);

        return await conn.QueryAsync<T>(sqlResult.Sql, sqlResult.NamedBindings);
    }

    public static Query SelectAllClassProperties(
        this Query query,
        Type classType,
        bool printLog = false
    )
    {
        var columnlists = new List<string>();
        var className = classType.Name;

        var instance = Activator.CreateInstance(classType);

        foreach (var classProperty in instance.GetType().GetProperties())
        {
            columnlists.Add($"{classType.Name}.{classProperty.Name}");
        }

        if (printLog)
        {
            UtilityExtension.ShowCurrentPosition(
                JsonConvert.SerializeObject(columnlists, Formatting.Indented)
            );
        }

        query.Select(columnlists);
        return query;
    }

    public static Query SelectAllClassProperties(
        this Query query,
        Type[] listClassType,
        bool printLog = false
    )
    {
        var columnlists = new List<string>();
        foreach (var classType in listClassType)
        {
            var className = classType.Name;

            var instance = Activator.CreateInstance(classType);

            foreach (var classProperty in instance.GetType().GetProperties())
            {
                columnlists.Add($"{className}.{classProperty.Name}");
            }
        }

        if (printLog)
        {
            UtilityExtension.ShowCurrentPosition(
                JsonConvert.SerializeObject(columnlists, Formatting.Indented)
            );
        }

        query.Select(columnlists);
        return query;
    }

    public static Query WithAutoAlias(
        this Query query,
        Query queryToRun,
        [CallerArgumentExpression(nameof(queryToRun))] string callerParameter = ""
    )
    {
        query.With(callerParameter, queryToRun);
        return query;
    }
}
