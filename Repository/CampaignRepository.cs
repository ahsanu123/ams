using System.Linq;
using System.Security.Claims;
using AMS.AppIdentity;
using AMS.Constant;
using AMS.Extension;
using AMS.Model;
using Microsoft.Data.Sqlite;
using Newtonsoft.Json;
using SqlKata;

namespace AMS.Repository;

public interface ICampaignRepository
{
    public Task<CampaignModel> UpdateCampaign(CampaignModel model);
    public Task CreateCampaign(CampaignModel model);
    public Task<IEnumerable<CampaignModel>> GetCampaigns(List<Claim> claims);
}

public class CampaignRepository : ICampaignRepository
{
    private readonly ISqliteConnectionProvider _conn;

    public CampaignRepository(ISqliteConnectionProvider connectionProvider)
    {
        _conn = connectionProvider;
    }

    private async Task CreateConnection(Func<SqliteConnection, Task> connection)
    {
        using (var conn = _conn.CreateConnection())
        {
            await connection(conn);
        }
    }

    public async Task<CampaignModel> UpdateCampaign(CampaignModel model)
    {
        CampaignModel? campaign = null;
        var GetCampaign_Query = new Query(nameof(CampaignModel)).Where(
            nameof(CampaignModel.Id),
            model.Id
        );

        var UpdateCampaign_Query = GetCampaign_Query.AsUpdate(model);

        await CreateConnection(async conn =>
        {
            await conn.ExecuteSqlKataAsync(UpdateCampaign_Query);
            campaign = await conn.QuerySingleSqlKataAsync<CampaignModel>(GetCampaign_Query);
        });

        return campaign;
    }

    public async Task CreateCampaign(CampaignModel model)
    {
        await CreateConnection(async conn =>
        {
            await conn.InsertToDatabase<CampaignModel>(model, true);
        });
    }

    public async Task<IEnumerable<CampaignModel>> GetCampaigns(List<Claim> claims)
    {
        IEnumerable<CampaignModel> campaigns = new List<CampaignModel>();

        var getAllCampaign_Query = new Query(nameof(CampaignModel));

        if (claims.Any(claim => claim.Value == RoleConstant.Manager))
        {
            await CreateConnection(async conn =>
            {
                campaigns = await conn.QuerySqlKataAsync<CampaignModel>(getAllCampaign_Query);
            });
        }
        else
        {
            await CreateConnection(async conn =>
            {
                foreach (var role in claims)
                {
                    var GetCampaignByType_Query = new Query(nameof(CampaignModel)).Where(
                        nameof(CampaignModel.Type),
                        role.Value
                    );
                    var campaign = await conn.QuerySqlKataAsync<CampaignModel>(
                        GetCampaignByType_Query
                    );
                    campaigns = [.. campaigns, .. campaign];
                }
            });
        }

        return campaigns;
    }
}
