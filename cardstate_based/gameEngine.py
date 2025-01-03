import numpy as np
from enum import Enum
from gameConstants import ECardStateEntries as csEntries, ESupportedPlayers as supportedPlayers
from gameState import gameState as gs

# Game Engine provides a valid move mask for the action space, checks that a valid action is requested, and performs said action on the game state returning the new game state.

class gameEngine:
    def __init__(self,
                 deckSize:   int = 60,
                 numPlayers: int = supportedPlayers.MAX_SUPPORTED_PLAYERS.value,
                 numPrizes:  int = 6):
        self.gameState = gs(deckSize = deckSize, numPlayers = numPlayers, numPrizes = numPrizes)
        
    def getValidActionMask(self,):
        return 0

    # called with action to perform.
    # checks to confirm action is a valid action
    # calls relevant method to peform said action (draw, attachEnergy, etc.)
    # returns updated game state
    def performAction(self,):
        return 0

    def draw(self,):
        return 0

    def attachEnergy(self,):
        return 0

    def playSupporter(self,):
        return 0

    def attack(self,):
        return 0
