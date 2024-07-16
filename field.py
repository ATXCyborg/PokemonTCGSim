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
    def __init__(self, numPlayers: int = 2):
        self.zones = {"deck": cardZone("deck")}
        for i in range(numPlayers):
            nameStr = f"p{i}_hand"
            self.zones[nameStr] = cardZone(nameStr)
        self.zones = {"discard": cardZone("discard")}
