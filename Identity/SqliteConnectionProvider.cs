using AMS.Model;
using Microsoft.Data.Sqlite;
using Microsoft.Extensions.Options;

namespace AMS.AppIdentity;

public interface ISqliteConnectionProvider
{
    public SqliteConnection CreateConnection();
}

public class SqliteConnectionProvider : ISqliteConnectionProvider
{
    private readonly ConnectionStrings _connString;

    public SqliteConnectionProvider(IOptions<ConnectionStrings> connString)
    {
        _connString = connString.Value;
    }

    public SqliteConnection CreateConnection()
    {
        return new SqliteConnection(_connString.Sqlite);
    }
}
