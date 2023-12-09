class CubeSet:
    def __init__(self, red=0, green=0, blue=0):
        self.red = red
        self.green = green
        self.blue = blue


class Game:
    def __init__(self, id, sets):
        self.id = id
        self.sets = sets


def parse_input(input_str):
    games = []
    for line in input_str.strip().split('\n'):
        parts = line.split(': ')
        game_id = int(parts[0].replace("Game ", ""))
        sets_str = parts[1].split('; ')
        sets = []
        for set_str in sets_str:
            cubes = CubeSet()
            for cube in set_str.split(', '):
                parts = cube.split(' ')
                count = int(parts[0])
                if parts[1] == 'red':
                    cubes.red = count
                elif parts[1] == 'green':
                    cubes.green = count
                elif parts[1] == 'blue':
                    cubes.blue = count
            sets.append(cubes)
        games.append(Game(game_id, sets))
    return games


def is_game_possible(game, bag):
    for set in game.sets:
        if set.red > bag['red'] or set.green > bag['green'] or set.blue > bag['blue']:
            return False
    return True


def find_minimum_cubes(game):
    min_red, min_green, min_blue = 0, 0, 0
    for set in game.sets:
        min_red = max(min_red, set.red)
        min_green = max(min_green, set.green)
        min_blue = max(min_blue, set.blue)
    return min_red, min_green, min_blue

def calculate_power(red, green, blue):
    return red * green * blue
        


def main():
    with open("./day_02_input.txt") as f:
        data = f.read().strip()
    
    games = parse_input(data)
    
    sum_ids = 0
    total_power = 0

    bag = {
        "red": 12,
        "green": 13,
        "blue": 14
    }

    for game in games:
        min_red, min_green, min_blue = find_minimum_cubes(game)
        power = calculate_power(min_red, min_green, min_blue)
        total_power += power
        print(f"Game {game.id}: Minimum cubes - Red: {min_red}, Green: {min_green}, Blue: {min_blue}, Power: {power}")
        if is_game_possible(game, bag):
            sum_ids += game.id

    print("Sum of possible game IDs:", sum_ids)
    print("Total power of minimum sets:", total_power)


if __name__ == "__main__":
    main()
