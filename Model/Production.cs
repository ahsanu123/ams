namespace AMS.Model;

public class Production
{
    public int Id { get; set; }

    public DateTime Date { get; set; }
    public double AmpasPrice { get; set; }
    public int Amount { get; set; }
    public string Description { get; set; } = "";
}
