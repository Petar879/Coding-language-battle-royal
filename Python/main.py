import requests
import random
import webbrowser

print("Select option")

while (True):
    inputOption = int(input())
    if(inputOption == 1):
        response = requests.get('https://api.senpy.club/v2/language/Python')
        jsonResponse = response.json()
        webbrowser.open(random.choice(jsonResponse))
    elif(inputOption == 2):
        response = requests.get('https://api.senpy.club/v2/languages')
        jsonResponse = response.json()
        selectedLanguage = random.choice(jsonResponse)
        response = requests.get('https://api.senpy.club/v2/language/' + selectedLanguage.replace("#", "%23").replace("+", "%2B"))
        webbrowser.open(random.choice(response.json()))
        # why was the first ever resolt Davinci with python book?
        # How did you know that I'm watching FGO while writting this code?
        # https://raw.githubusercontent.com/cat-milk/Anime-Girls-Holding-Programming-Books/master/Python/Davinci_Python_magic.png