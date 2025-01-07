using AMS.AppIdentity;
using AMS.Extension;
using AMS.Model;
using Microsoft.Data.Sqlite;
using SqlKata;

namespace AMS.Repository;

public interface IProductionRepository
{
    public Task<AMSResult> AddProduction(int amount, DateTime date);
    public Task<ProductionTotalError> GetProductionTotalError(DateTime date);
}

public class ProductionRepositoy : IProductionRepository
{
    private readonly ISqliteConnectionProvider _conn;

    public ProductionRepositoy(ISqliteConnectionProvider connectionProvider)
    {
        _conn = connectionProvider;
    }

    private async Task _createConnection(Func<SqliteConnection, Task> connection)
    {
        using (var conn = _conn.CreateConnection())
        {
            await connection(conn);
        }
    }

    private async Task<double> _getCurrentAmpasPrice()
    {
        double price = 0;
        var GetCurrentAmpasPrice_Query = new Query(nameof(AmpasPrice)).OrderByDesc(
            nameof(AmpasPrice.ChangedTime)
        );
        await _createConnection(async conn =>
        {
            var prices = await conn.QuerySqlKataAsync<AmpasPrice>(GetCurrentAmpasPrice_Query);
            price = prices.ToList()[0].Price;
        });
        return price;
    }

    public async Task<AMSResult> AddProduction(int amount, DateTime date)
    {
        var result = new AMSResult { Success = false };
        var currentAmpasPrice = await _getCurrentAmpasPrice();

        var production = new Production
        {
            Date = DateTime.Now,
            Amount = amount,
            Description = $"{DateTime.Now.ToLongDateString()}, production {amount}",
            AmpasPrice = currentAmpasPrice,
        };
        await _createConnection(async conn =>
        {
            await conn.InsertToDatabase(production, true);
            result.Success = true;
        });
        return result;
    }

    public async Task<ProductionTotalError> GetProductionTotalError(DateTime date)
    {
        var productionError = new ProductionTotalError();
        var currentAmpasPrice = await _getCurrentAmpasPrice();

        var GetAllTakenAmpasForMonth_Query = new Query(nameof(AmpasModel))
            .WhereDatePart("year", nameof(AmpasModel.TakenTime), date.ToString("yyyy"))
            .WhereDatePart("month", nameof(AmpasModel.TakenTime), date.ToString("MM"));

        var GetAllProductionforMonth_Query = new Query(nameof(Production))
            .WhereDatePart("year", nameof(Production.Date), date.ToString("yyyy"))
            .WhereDatePart("month", nameof(Production.Date), date.ToString("MM"));

        await _createConnection(async conn =>
        {
            var allAmpas = await conn.QuerySqlKataAsync<AmpasModel>(GetAllTakenAmpasForMonth_Query);
            var allProduction = await conn.QuerySqlKataAsync<Production>(
                GetAllProductionforMonth_Query
            );

            var totalAmpasTaken = allAmpas.Select(item => item.Amount).Sum();
            var totalProduction = allProduction.Select(item => item.Amount).Sum();

            var totalAmpasWorth = allAmpas.Select(item => (item.Amount * item.Price)).Sum();
            var totalProductionWorth = allProduction
                .Select(item => (item.Amount * item.AmpasPrice))
                .Sum();

            productionError.TotalSettedByUser = totalAmpasTaken;
            productionError.TotalProduction = totalProduction;

            productionError.TakenError = totalProduction - totalAmpasTaken;
            productionError.WorthError = totalProductionWorth - totalAmpasWorth;

            productionError.TotalTakenWorth = totalAmpasWorth;
            productionError.TotalProductionWorth = totalProductionWorth;
        });

        return productionError;
    }
}
