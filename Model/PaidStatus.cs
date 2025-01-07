namespace AMS.Model;

public class PaidStatus
{
    public string Username { get; set; }
    public DateTime From { get; set; }
    public DateTime To { get; set; }
    public bool Paid { get; set; }
}
