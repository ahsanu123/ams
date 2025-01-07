namespace AMS.Model;

public class AmpasModel
{
    public int Id { get; set; }
    public int UserId { get; set; }
    public bool Paid { get; set; }
    public DateTime ProductionDate { get; set; }
    public DateTime TakenTime { get; set; }
    public double Price { get; set; }
    public int Amount { get; set; }
    public string Description { get; set; } = "";
}
