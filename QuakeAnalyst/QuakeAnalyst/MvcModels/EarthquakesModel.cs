using QuakeAnalyst.Repo;

namespace QuakeAnalyst.MvcModels
{
    public class EarthquakesModel
    {
        public EarthquakesModel()
        {
            Earthquakes = new List<Earthquake>();
        }
        public List<Earthquake> Earthquakes { get; set; }
    }
   
}
