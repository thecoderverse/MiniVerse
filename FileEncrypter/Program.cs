// See https://aka.ms/new-console-template for more information
using FileHasher;
using System.Reflection.Metadata;

Console.WriteLine("Hello, World!");

Console.WriteLine("Chose an action:");
Console.WriteLine("1. Encrypt data");
Console.WriteLine("2. Decrypt data");
int opt = InputHelpers.RequestEntryOption("Enter an option number", new int[] { 1, 2 });

switch (opt)
{
    case 1:
        Encrypter e = new Encrypter();
        e.Run();
        break;
    case 2:
        Decrypter d = new Decrypter();
        d.Run();
        break;
}







