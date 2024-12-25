import numpy as np
from enum import Enum
from gameconstants import ECardStateEntries, ESupportedPlayers

class gameState:
    def __init__(self,
                 deckSize: int = 60,
                 numPlayers: int = ESupportedPlayers.MAX_SUPPORTED_PLAYERS.value,
                 numPrizes: int = 6):
        self.deckSize = deckSize
        self.numPlayers = numPlayers
        self.cardState = np.zeros( (self.numPlayers*self.deckSize,
                                   ECardStateEntries.CARD_STATE_MAX_SIZE.value),
                                   dtype=np.int16)
        self.numPrizes = numPrizes
        self.prizesRemaining = np.zeros(self.numPlayers, dtype=np.int16) + self.numPrizes
        self.turnPlayer = ESupportedPlayers.PLAYER_1.value

    def loadDeck(self, player: int = 0, deckList: list = []):
        if (player < ESupportedPlayers.MAX_SUPPORTED_PLAYERS.value):
            if len(deckList) == self.deckSize:
                for i in range(self.deckSize):
                    self.cardState[(player * self.deckSize)+i,
                                   ECardStateEntries.UID.value] = deckList[i]
                
                if player == ESupportedPlayers.PLAYER_1.value:
                    self.cardState[(player * self.deckSize):
                                   (player * self.deckSize)+self.deckSize,
                                   ECardStateEntries.P1V_UID.value] = 1
                elif player == ESupportedPlayers.PLAYER_2.value:
                    self.cardState[(player * self.deckSize):
                                   (player * self.deckSize)+self.deckSize,
                                   ECardStateEntries.P2V_UID.value] = 1

            else:
                print(f"Invalid size of deck {len(deckList)} provided."+
                      f" Must be {self.deckSize} in length.\n")
        else:
            print(f"Invalid player {player} specified."+
                  f" Only {ESupportedPlayers.MAX_SUPPORTED_PLAYERS.value} supported.\n")
        return

    def printDeckList(self, player: int = 0):
        if (player < ESupportedPlayers.MAX_SUPPORTED_PLAYERS.value):
            print(self.cardState[(player * self.deckSize):(player * self.deckSize)+self.deckSize,
                                 ECardStateEntries.UID.value])
        else:
            print(f"Invalid player {player} specified."+
                  f" Only {ESupportedPlayers.MAX_SUPPORTED_PLAYERS.value} supported.\n")
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
