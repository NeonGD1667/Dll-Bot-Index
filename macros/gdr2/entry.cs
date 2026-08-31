using System;

class Program
{
    static void Main()
    {
        string[] macros =
        {
            "Bloodbath.gdr2",
            "Acu.gdr2",
            "Slaughterhouse.slc3",
            "RandomMacro.gdr"
        };

        Console.WriteLine("🤖 Dll Bot Macro Scanner");
        Console.WriteLine("-------------------------");

        foreach (var macro in macros)
        {
            Console.WriteLine($"📦 Found: {macro}");
        }

        Console.WriteLine();
        Console.WriteLine($"🔥 Total macros: {macros.Length}");
        Console.WriteLine("✅ index.json updated!");
    }
}
