using Microsoft.AspNetCore.Mvc;
using QuakeAnalyst.ApiService;
using QuakeAnalyst.Models;
using System.Diagnostics;

namespace QuakeAnalyst.Controllers
{
    public class HomeController : Controller
    {
        private readonly ILogger<HomeController> _logger;
        private readonly OrhanAydogduApiHandler _apiHandler;

        public HomeController(ILogger<HomeController> logger, OrhanAydogduApiHandler ApiHandler)
        {
            _logger = logger;
            _apiHandler = ApiHandler;
        }

        public IActionResult Index()
        {
            return View();
        }

        public IActionResult Privacy()
        {
            return View();
        }
        public async Task<IActionResult> GeoLocations()
        {
            var model = new GeoLocationsModel();
            model.GeoLocations = await _apiHandler.GetGeoLocations();
            return View(model);
        }

        [ResponseCache(Duration = 0, Location = ResponseCacheLocation.None, NoStore = true)]
        public IActionResult Error()
        {
            return View(new ErrorViewModel { RequestId = Activity.Current?.Id ?? HttpContext.TraceIdentifier });
        }
    }
}