from types import SimpleNamespace

Spell = lambda *p: SimpleNamespace(cost=p[0], damage=p[1], heal=p[2], effect=p[3], name=p[4])
Effect = lambda *p: SimpleNamespace(length=p[0], armor=p[1], damage=p[2], mana=p[3], name=p[4])

spells = {
    "Missle":   (53,  4, 0, None,                'Missle'),
    "Drain":    (73,  2, 2, None,                'Drain'),
    "Shield":   (113, 0, 0, (6, 7, 0, 0,   'S'), 'Shield'),
    "Poison":   (173, 0, 0, (6, 0, 3, 0,   'P'), 'Poison'),
    "Recharge": (229, 0, 0, (5, 0, 0, 101, 'R'), 'Recharge')
}

DEBUG = False

def debug(*args):
    if DEBUG:
        print(*args)

def play(castlist):
    player = SimpleNamespace(hp=50, mana=500, effects=[])
    boss = SimpleNamespace(hp=58, damage=9)

    bossturn = False
    active = {
        'S': False,
        'P': False,
        'R': False
    }
    turn = 0
    debug("\n--- N E W   G A M E ---\n")
    debug("Spells to cast:", '; '.join(map(lambda x: x.name, castlist)))
    castcost = 0
    while True:
        turn += 1
        debug("TURN-[{}], BOSS hp: {}, dmg:{}, P hp:{} mana:{} effects: {} ".format(turn, boss.hp, boss.damage, player.hp, player.mana, player.effects))
        if not bossturn:
            player.hp -= 1
            if player.hp <= 0:
                return "LOSS"
        playerarmor = 0
        new_effects = []
        for effect in player.effects:
            boss.hp -= effect.damage
            player.mana += effect.mana
            playerarmor += effect.armor
            debug("\teffect: ", effect.name)
            if effect.length > 1:
                effect.length -= 1
                active[effect.name] = True
                new_effects.append(effect)
            else:
                active[effect.name] = False
        player.effects = new_effects
        if boss.hp <= 0:
            debug("WIN")
            return "WIN"
        if bossturn:
            player.hp -= max(1, boss.damage - playerarmor)
            if player.hp <= 0:
                debug("Player died")
                return "LOSS"
        else:
            if not castlist:
                debug("list too short")
                return "SHORT"
            spell = castlist[0]
            castlist = castlist[1:]
            player.mana -= spell.cost
            if player.mana < 0:
                debug("Not enough mana", spell)
                return "LOSS"
            debug("\tcast ~~", spell)

            castcost += spell.cost
            debug("current castcost:", castcost)

            player.hp += spell.heal
            boss.hp -= spell.damage
            if boss.hp <= 0:
                debug("WIN")
                return "WIN"
            if spell.effect:
                if active[spell.effect.name]:
                    debug("Tried to use active effect", spell)
                    return "LOSS"
                player.effects.append(spell.effect)
        bossturn = not bossturn

spelllists = [[s]  for s in  spells.keys()]
mincost = 9999999

while spelllists:
    newlists = []
    for spelllist in spelllists:
        for spell in spells.keys():
            namelist = [s for s in spelllist + [spell]]
            castlist = [Spell(*spells[s]) for s in namelist]
            for s in castlist:
                if s.effect:
                    s.effect = Effect(*s.effect)
            if sum([s.cost for s in castlist]) > mincost:
                continue
            result = play(castlist)
            if result == "SHORT":
                newlists.append(namelist)
            elif result == "LOSS":
                pass
            elif result == "WIN":
                cost = sum([x.cost for x in castlist])
                if cost < mincost:
                    mincost = cost
                print("WIN!", mincost, cost, [x.name for x in castlist])
    spelllists = newlists