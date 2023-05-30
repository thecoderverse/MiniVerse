using AgileTips.Api.Models;
using Microsoft.EntityFrameworkCore;

namespace AgileTips.Api.Data;

public class ApiDbContext
       : DbContext
{
    protected readonly IConfiguration Configuration;
    public ApiDbContext(IConfiguration configuration)
    {
        Configuration = configuration;
    }

    protected override void OnConfiguring(DbContextOptionsBuilder options)
    {
        options.UseNpgsql(Configuration.GetConnectionString("ConStr"));
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<Scope>().HasData(
                   new Scope { Id = 1, Name = "Scrum" },
                   new Scope { Id = 2, Name = "Kanban" }

        );
    }
    public DbSet<Scope> Scopes { get; set; }
    public DbSet<Tip> Tips { get; set; }
}