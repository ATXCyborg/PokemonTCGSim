import cards

test_cards = []

test_cards.append(cards.pokemonCard(cardName = "Metagross", subTypes=["Evolution"]))
test_cards.append(cards.itemCard(cardName="Rare Candy"))
test_cards.append(cards.supporterCard(cardName="Iono"))
test_cards.append(cards.pokemonCard(cardName="Comfey", subTypes=["Basic"]))

print(test_cards)
print(test_cards[0].isPokemon())
print(test_cards[1].isSubType("Supporter"))
print(test_cards[2].isSubType("Supporter"))
print(test_cards[3].subTypes)
print(test_cards[0].subTypes)
