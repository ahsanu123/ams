namespace AMS.Model;

public class AmpasSummary
{
    public int TotalTaken { get; set; }
    public string Username { get; set; }
    public double TotalTakenPrice { get; set; }
    public List<KeyValuePair<string, int>> UserTakenCount { get; set; }
}
