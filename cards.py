class baseCard:
    def __init__(self, cardName: str = None, cardType: str = None, subTypes: list = []):
        self.cardName = cardName
        self.cardType = cardType
        self.subTypes = subTypes

    def printName(self):
        print(self.cardName)

    def isSubType(self, sub: str):
        return sub in self.subTypes

    def isPokemon(self):
        return self.cardType == "Pokemon"

    def isTrainer(self):
        return self.cardType == "Trainer"

    def isEnergy(self):
        return self.cardType == "Energy"


########################## Pokemon Cards ######################################
class pokemonCard(baseCard):
    def __init__(self, cardName: str = None,
                 subTypes: list = [],
                 attacks: list = [],
                 hp: int = 0,
                 pokeType: str = None,
                 weakness: str = None,
                 resistance: str = None):
        super().__init__(cardName = cardName, cardType = "Pokemon", subTypes = subTypes)
        self.attacks = attacks
        self.ability = None
        self.hp = hp
        self.type = pokeType
        self.weakness = weakness
        self.resistance = resistance


########################## Trainer Cards ######################################
class trainerCard(baseCard):
    def __init__(self, cardName: str = None, subTypes: list = []):
        super().__init__(cardName = cardName, cardType = "Trainer", subTypes = subTypes)

class itemCard(trainerCard):
    def __init__(self, cardName: str = None, subTypes: list = []):
        subTypes.append("Item")
        super().__init__(cardName = cardName, subTypes = subTypes)

class toolCard(trainerCard):
    def __init__(self, cardName: str = None, subTypes: list = []):
        subTypes.append("Tool")
        super().__init__(cardName = cardName, subTypes = subTypes)

class supporterCard(trainerCard):
    def __init__(self, cardName: str = None, subTypes: list = []):
        subTypes.append("Supporter")
        super().__init__(cardName = cardName, subTypes = subTypes)

########################### Energy Cards ######################################
class energyCard(baseCard):
    def __init__(self, cardName: str = None, subTypes: list = []):
        super().__init__(cardName = cardName, cardType = "Energy", subTypes = subTypes)

class basicEnergyCard(energyCard):
    def __init__(self, cardname: str = None, subTypes: list = []):
        subTypes.append("Basic")
        super().__init__(cardName = cardName, subTypes = subTypes)

class specialEnergyCard(energyCard):
    def __init__(self, cardName: str = None, subTypes: list = []):
        subTypes.append("Special")
        super().__init__(cardName = cardName, subTypes = subTypes)
