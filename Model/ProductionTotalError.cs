namespace AMS.Model;

public class ProductionTotalError
{
    public int TotalProduction { get; set; }
    public int TotalSettedByUser { get; set; }

    public double TotalProductionWorth { get; set; }
    public double TotalTakenWorth { get; set; }

    // TotalProduction - TotalSettedByUser
    public int TakenError { get; set; }
    public double WorthError { get; set; }
}
