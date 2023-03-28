using System.Diagnostics;
using System.Text;
using System.Text.Json;

HttpClient client = new();
Random rnd = new Random();

Console.WriteLine("Select option");
while(true)
{
    switch(int.Parse(Console.ReadLine()))
    {
        //Open a random C# image
        case 1:
            var json = await client.GetStringAsync("https://api.senpy.club/v2/language/C%23");
            string[] urls = JsonSerializer.Deserialize<string[]>(json);
            //The '#' part gets ignored, replaced with %23
            urls = urls.Select(x => x.Replace("C#","C%23")).ToArray();
            //Opens link in default browser
            Process.Start(new ProcessStartInfo(urls[rnd.Next(0, urls.Count())]) { UseShellExecute = true });
            break;
        
        case 2:
            var jsonLanguages = await client.GetStringAsync("https://api.senpy.club/v2/languages");
            string[] languageArray = JsonSerializer.Deserialize<string[]>(jsonLanguages );
            string language = languageArray[rnd.Next(0, languageArray.Count())];
            language = new StringBuilder(language).Replace("#", "%23").Replace("+", "%2B").ToString();

            jsonLanguages = await client.GetStringAsync("https://api.senpy.club/v2/language/" + language);
            string[] selectedLanguageLinks = JsonSerializer.Deserialize<string[]>(jsonLanguages);

            Process.Start(new ProcessStartInfo(selectedLanguageLinks[rnd.Next(0, selectedLanguageLinks.Count())]) { UseShellExecute = true });
            break;
    }
}