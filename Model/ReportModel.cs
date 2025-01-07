namespace AMS.Model;

public class ReportModel
{
    public int Id { get; set; }
    public int Title { get; set; }
    public DateTime Time { get; set; }
    public string Description { get; set; } = "";
    public string Data { get; set; } = "";
}
