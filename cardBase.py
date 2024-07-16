class baseCard:
    def __init__(self, cardName: str = None):
        self.cardName = cardName

    def printName(self):
        print(self.cardName)
