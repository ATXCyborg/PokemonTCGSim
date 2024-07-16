import cardBase as cards
from random import shuffle

class cardZone:
    def __init__(self, name: str = None):
        self.name = name
        self.cards = []

    def load(self, cardsToAdd = []):
        for card in cardsToAdd:
            self.cards.append(card)

    def shuffle(self):
        shuffle(self.cards)

    def draw(self, numToDraw: int = 1):
        ret = None

        if numToDraw >= 1:
            ret = self.cards[:numToDraw]
            self.cards = self.cards[numToDraw:]

        return ret
    
class field:
    zoneNames = ["deck", "discard"]
    zones = {}

    def __init__(self, numPlayers: int = 2):
        for i in range(numPlayers):
            nameStr = f"p{i}_hand"
            self.zoneNames.append(nameStr)

        for name in self.zoneNames:
            self.zones[name] = cardZone(name)
            
