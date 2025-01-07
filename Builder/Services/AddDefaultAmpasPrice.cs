using AMS.AppIdentity;
using AMS.Extension;
using AMS.Model;
using SqlKata;

namespace AMS.Services;

public static class DefaultAmpasPriceBuilder
{
    public static async Task<IApplicationBuilder> AddDefaultAmpasPrice(
        this IApplicationBuilder builder,
        double price = 11000
    )
    {
        using var scope = builder.ApplicationServices.CreateScope();

        var sqliteConnection = scope.ServiceProvider.GetService<ISqliteConnectionProvider>();

        if (sqliteConnection == null)
            throw new ArgumentNullException("sqliteConnection is Null");

        var CheckIfDefaultPriceExists = new Query(nameof(AmpasPrice));

        using (var conn = sqliteConnection.CreateConnection())
        {
            var listPrice = await conn.QuerySqlKataAsync<AmpasPrice>(CheckIfDefaultPriceExists);
            if (listPrice.ToList().Count == 0)
            {
                var defaultPrice = new AmpasPrice { Price = price, ChangedTime = DateTime.Now };
                await conn.InsertToDatabase(defaultPrice, true);
            }
        }

        return builder;
    }
}
