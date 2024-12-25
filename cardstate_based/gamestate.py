import numpy as np
from enum import Enum

class ECardStateEntries(Enum):
    UID      = 0
    LOC_NAME = 1
    LOC_NUM  = 2
    P1V_UID  = 3
    P1V_LOC  = 4
    P2V_UID  = 5
    P2V_LOC  = 6
    CARD_STATE_MAX_SIZE = 7

class ESupportedPlayers(Enum):
    PLAYER_1 = 0
    PLAYER_2 = 1
    MAX_SUPPORTED_PLAYERS = 2

class gameState:
    def __init__(self, deckSize: int = 60):
        self.deckSize = deckSize
        self.numPlayers = 2
        self.cardState = np.zeros( (self.numPlayers*self.deckSize,
                                   ECardStateEntries.CARD_STATE_MAX_SIZE.value),
                                   dtype=np.int16)
        self.numPrizes = 6
        self.prizesRemaining = np.zeros(self.numPlayers, dtype=np.int16) + self.numPrizes
        self.turnPlayer = ESupportedPlayers.PLAYER_1.value

    def loadDeck(self, player: int = 0, deckList: list = []):
        if (player < ESupportedPlayers.MAX_SUPPORTED_PLAYERS.value):
            if len(deckList) == self.deckSize:
                for i in range(self.deckSize):
                    self.cardState[(player * self.deckSize)+i, ECardStateEntries.UID.value] = deckList[i]
            else:
                print(f"Invalid size of deck {len(deckList)} provided. Must be {self.deckSize} in length.\n")
        else:
            print(f"Invalid player {player} specified. Only {ESupportedPlayers.MAX_SUPPORTED_PLAYERS.value} supported.\n")
        return

    def printDeckList(self, player: int = 0):
        if (player < ESupportedPlayers.MAX_SUPPORTED_PLAYERS.value):
            print(self.cardState[(player * self.deckSize):(player * self.deckSize)+self.deckSize, ECardStateEntries.UID.value])
        else:
            print(f"Invalid player {player} specified. Only {ESupportedPlayers.MAX_SUPPORTED_PLAYERS.value} supported.\n")
        return

    def printCardState(self):
        print(self.cardState)
        return

    def getGameState(self):
        return (self.cardState,self.prizesRemaining,self.turnPlayer)

    def loadGameState(self, deckSize, numPlayers, cardState, numPrizes,
                      prizesRemaining, turnPlayer):
        self.deckSize        = deckSize
        self.numPlayers      = numPlayers
        self.cardState       = cardState
        self.numPrizes       = numPrizes
        self.prizesRemaining = prizesRemaining
        self.turnPlayer      = turnPlayer

    def exportGameState(self):
        return (self.deckSize, self.numPlayers, self.cardState, self.numPrizes,
                self.prizesRemaining, self.turnPlayer)
