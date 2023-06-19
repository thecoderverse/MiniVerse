using Newtonsoft.Json;
using System;
using System.Security.Cryptography.X509Certificates;

public class Geolocation
{
    [JsonProperty("type")]
    public string Type { get; set; }
    [JsonProperty("coordinates")]
    public Coordinates Coordinates { get; set; }
}
