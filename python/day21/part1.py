from math import ceil 
from sys import maxsize
from itertools import combinations

boss_hp  = 100
boss_arm = 2
boss_dmg = 8

weapons = [(8,4), (10,5), (25,6), (40,7), (74,8)]
armors = [(13,1), (31,2), (53,3), (75,4), (102,5), (0,0)]
rings = [(25,1,0), (50,2,0), (100,3,0), (20,0,1), (40,0,2), (80,0,3), (0,0,0), (0,0,0)]
 
#Weapons:    Cost  Damage  Armor
#Dagger        8     4       0
#Shortsword   10     5       0
#Warhammer    25     6       0
#Longsword    40     7       0
#Greataxe     74     8       0
#
#Armor:      Cost  Damage  Armor
#Leather      13     0       1
#Chainmail    31     0       2
#Splintmail   53     0       3
#Bandedmail   75     0       4
#Platemail   102     0       5
#
#Rings:      Cost  Damage  Armor
#Damage +1    25     1       0
#Damage +2    50     2       0
#Damage +3   100     3       0
#Defense +1   20     0       1
#Defense +2   40     0       2
#Defense +3   80     0       3

def win(hp, armor, damage, boss_hp, boss_arm, boss_dmg):
    loseturns = ceil(hp / max((boss_dmg - armor), 1))
    winturns = ceil(boss_hp / max((damage - boss_arm), 1))
    return winturns <= loseturns 
    
player_hp = 100 
    
min_cost = maxsize
max_cost = 0

for w in weapons:
    for a in armors:
        for r1, r2 in combinations(rings, 2):
            cost = 0
            damage = 0
            armor = 0
            
            cost += w[0]
            damage += w[1]
             
            cost += a[0]
            armor += a[1]
            
            cost += r1[0]
            damage += r1[1]
            armor += r1[2]
            
            cost += r2[0]
            damage += r2[1]
            armor += r2[2]
            
            if win(player_hp, armor, damage, boss_hp, boss_arm, boss_dmg):
                min_cost = min(min_cost, cost)
            else:
                max_cost = max(max_cost, cost)

print("Answer 1:", min_cost)
print("Answer 2:", max_cost)
