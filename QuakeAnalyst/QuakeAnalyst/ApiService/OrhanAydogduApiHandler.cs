using Microsoft.AspNetCore.Http;
using Newtonsoft.Json;
using NLog;
using QuakeAnalyst.Repo;
using System.Text.Json.Nodes;

namespace QuakeAnalyst.ApiService
{
    public class OrhanAydogduApiHandler
    {
        private static readonly Logger _logger = LogManager.GetCurrentClassLogger();
        string _geoLocationsQueryString = "https://api.orhanaydogdu.com.tr/deprem/statics/cities";
        List<GeoLocation> _geoLocations = new List<GeoLocation>();
        DateTime _lastGeoLocationUpdate = DateTime.MinValue;
        TimeSpan _validUpdateSpan = new TimeSpan(1,0,0,0);
        public OrhanAydogduApiHandler()
        {
            QueryGeoLocations();
        }
        public async Task<List<GeoLocation>> GetGeoLocations()
        {
            bool res;
            bool isDataAvailable = _geoLocations?.Count > 0;
            bool isDataUpToDate = (DateTime.Now - _lastGeoLocationUpdate) > _validUpdateSpan;
            if(isDataAvailable && isDataUpToDate)
            {
                return _geoLocations ?? new List<GeoLocation>(); 
            }
            else
            {
                res = await QueryGeoLocations();
            }
            return _geoLocations ?? new List<GeoLocation>();
        }

        private async Task<bool> QueryGeoLocations()
        {
            HttpClient client = new HttpClient();
            try
            {
                using HttpResponseMessage response = await client.GetAsync(_geoLocationsQueryString);
                response.EnsureSuccessStatusCode();
                string responseBody = await response.Content.ReadAsStringAsync();

                CityQueryResult qResult = JsonConvert.DeserializeObject<CityQueryResult>(responseBody);
                if (qResult is null)
                {
                    return false;
                    _logger.Info($"GeoLocation list returned empty from the api endpoint. HTTP Request was : {_geoLocationsQueryString}");
                }
                _geoLocations = qResult.result;
                _lastGeoLocationUpdate = DateTime.Now;
                return true;
            }
            catch (HttpRequestException e)
            {
                Console.WriteLine("\nException Caught!");
                Console.WriteLine("Message :{0} ", e.Message);
                return false;
            }
           
        }
    }

    public class CityQueryResult
    {
        public bool status;
        public int httpStatus;
        public int serverloadms;
        public string desc;
        public List<GeoLocation> result;
    }
}
