namespace AMS.InternalMigration;

using System.Data;
using System.Text.RegularExpressions;
using FluentMigrator;
using FluentMigrator.Builders.Create.Table;

public static class ModelToMigration
{
    public static ICreateTableColumnOptionOrWithColumnSyntax IsNullable(
        this ICreateTableColumnOptionOrWithColumnSyntax withColumnSyntax,
        string typeInfo
    )
    {
        if (typeInfo.Contains(nameof(System.Nullable)))
            withColumnSyntax.Nullable();
        return withColumnSyntax;
    }

    public static Migration GenerateForeignKey(this Migration migration, Type modelType)
    {
        var tableName = modelType.FullName.Split('.').Last();

        foreach (var key in modelType.GetProperties())
        {
            var typeString = key.ToString();
            var isPrimaryKey = Regex.IsMatch(key.Name.ToLower(), "^id$");
            var isForeignKey = Regex.IsMatch(key.Name, "Id$");

            if (isForeignKey && !isPrimaryKey)
            {
                migration
                    .Create.ForeignKey()
                    .FromTable(tableName)
                    .ForeignColumn(key.Name)
                    .ToTable(key.Name.Substring(0, key.Name.Length - 2))
                    .PrimaryColumn("Id")
                    .OnDelete(Rule.Cascade)
                    .OnDeleteOrUpdate(Rule.Cascade);
            }
        }

        return migration;
    }

    public static Migration ConvertModelToMigration(
        this Migration migration,
        Type modelType,
        IEnumerable<string> excludedTypes = null
    )
    {
        var tableName = modelType.FullName.Split('.').Last();

        var table = migration.Create.Table(tableName);

        foreach (var key in modelType.GetProperties())
        {
            var typeString = key.ToString();
            Console.WriteLine(typeString);

            if (excludedTypes != null)
            {
                Console.WriteLine($"{tableName}.{key.Name} = {typeString}");
                var excluded = excludedTypes.Any(item => typeString.Contains(item));
                if (excluded)
                    continue;
            }

            var column = table.WithColumn(key.Name);
            var isPrimaryKey = Regex.IsMatch(key.Name.ToLower(), "^id$");
            var isForeignKey = Regex.IsMatch(key.Name, "Id$");

            if (isPrimaryKey)
            {
                column.AsInt32().Nullable().Identity().PrimaryKey();
                continue;
            }
            if (isForeignKey)
            {
                column.AsInt32().IsNullable(typeString);
                continue;
            }

            if (typeString.Contains(nameof(System.String)))
            {
                // column.AsString().IsNullable(typeString);
                column.AsString().Nullable();
                continue;
            }

            if (typeString.Contains(nameof(Boolean)))
            {
                column.AsBoolean().IsNullable(typeString);
                continue;
            }

            if (typeString.Contains(nameof(System.DateTime)))
            {
                column.AsDate().IsNullable(typeString);
                continue;
            }

            if (typeString.Contains(nameof(Double)))
            {
                column.AsDouble().IsNullable(typeString);
                continue;
            }

            if (typeString.Contains(nameof(System.Int16)))
            {
                column.AsInt16().IsNullable(typeString);
                continue;
            }

            if (typeString.Contains(nameof(System.Int32)))
            {
                column.AsInt32().IsNullable(typeString);
                continue;
            }

            if (typeString.Contains(nameof(System.Int64)))
            {
                column.AsInt64().IsNullable(typeString);
                continue;
            }

            if (typeString.Contains(nameof(System.Single)))
            {
                column.AsFloat().IsNullable(typeString);
                continue;
            }
        }

        return migration;
    }
}
