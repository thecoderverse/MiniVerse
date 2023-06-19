using Newtonsoft.Json;

namespace QuakeAnalyst.Repo
{
    public class Earthquake
    {
        [JsonProperty("_id")]
        public string Id { get; set; }
        [JsonProperty("earhtquake_id")]
        public string Earthquake_Id { get; set; }
        [JsonProperty("title")]
        public string Title { get; set; }
        [JsonProperty("date")]
        public DateTime Date { get; set; }
        [JsonProperty("mag")]
        public double Magnitude { get; set; }
        [JsonProperty("depth")]
        public double Depth { get; set; }
        [JsonProperty("geoJson")]
        public Geolocation Geolocation { get; set; }
    }
}
