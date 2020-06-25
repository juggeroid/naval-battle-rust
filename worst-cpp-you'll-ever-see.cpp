

#include <iostream>
#include <vector>
#include <array>
#include <random>
#include <chrono>

namespace {
    static constexpr auto FIELD_SIZE = 10;
    enum CellType {
      EMPTY,
      UNAVAILABLE,
      OCCUPIED
    };
    using Coordinates = std::pair<std::int32_t, std::int32_t>;
    using Directions = std::array<Coordinates, 9>;

    static std::minstd_rand generator{ std::random_device{}() };
    static std::uniform_int_distribution distribution(0, FIELD_SIZE);
    static std::uniform_int_distribution bool_distribution(0, 1);
}

struct Field
{
  std::array<CellType, FIELD_SIZE* FIELD_SIZE> field = { EMPTY };
  friend std::ostream& operator<<(std::ostream& o, const Field& field)
  {
    for (size_t index = 0; index < FIELD_SIZE * FIELD_SIZE;) {
      switch (field.field[index]) {
        case EMPTY:
          o << ".";
          break;
        case UNAVAILABLE:
          o << " ";
          break;
        case OCCUPIED:
          o << "X";
          break;
      }
      if (++index % FIELD_SIZE == 0)
        o << "\n";
    }
    return o;
  }
};

bool
is_valid_formation(const Field& field, std::size_t x, std::size_t y,
                   std::int32_t dx, std::int32_t dy,
                   std::size_t ship_size) noexcept
{

  static constexpr auto DIRECTIONS = Directions {{{  0,  0 },
                                                  {  0,  1 },
                                                  {  0, -1 },
                                                  { -1,  0 },
                                                  {  1,  0 },
                                                  { -1,  1 },
                                                  {  1, -1 },
                                                  { -1, -1 },
                                                  {  1,  1 }}};

  const auto within_bounds = [&]
  (const auto x, const auto y, const std::pair<std::int32_t, std::int32_t>& direction = {0, 0})
  {
    const bool xbound = (x + direction.first  < FIELD_SIZE) && (x + direction.first  >= 0);
    const bool ybound = (y + direction.second < FIELD_SIZE) && (y + direction.second >= 0);
    return xbound && ybound;
  };

  for (std::size_t iteration = 0; iteration < ship_size; ++iteration) {
    const std::int32_t nx = x + (dx * iteration);
    const std::int32_t ny = y + (dy * iteration);
    for (const auto& direction: DIRECTIONS) {
      if (!within_bounds(nx, ny, direction)) continue;
      const auto& bounding_box_cell = field.field[(nx + direction.first) + ((ny + direction.second) * FIELD_SIZE)];
      if (OCCUPIED == bounding_box_cell) return false;
    }
  }

  for (std::size_t iteration = 0; iteration < ship_size; ++iteration) {
    if (!within_bounds(x, y)) return false;
    const auto& current_cell = field.field[(y * FIELD_SIZE) + x];
    if (OCCUPIED == current_cell || UNAVAILABLE == current_cell) return false;
    x += dx;
    y += dy;
  }
  return true;
}

void
get_available_cells(const Field& field, std::int32_t dx, std::int32_t dy,
                    std::size_t ship_size,
                    std::vector<Coordinates>& buffer) noexcept
{
  buffer.clear();
  for (std::size_t x = 0; x < FIELD_SIZE; ++x) {
    for (std::size_t y = 0; y < FIELD_SIZE; ++y) {
      if (is_valid_formation(field, x, y, dx, dy, ship_size))
        buffer.push_back({ x, y });
    }
  }
}

auto emplace_ships(Field& field, std::size_t ship_size,
                   std::vector<Coordinates>& buffer) noexcept
{
  const auto get_alignment = [&] {
    return bool_distribution(generator) ? Coordinates {1, 0}
                                        : Coordinates {0, 1};
  };

  const auto pick_random = [&](auto begin, auto end) noexcept
  {
    std::uniform_int_distribution<> distribution(0, std::distance(begin, end) - 1);
    std::advance(begin, distribution(generator));
    return begin;
  };

  auto [dx, dy] = get_alignment();
  get_available_cells(field, dx, dy, ship_size, buffer);
  auto [x, y] = *pick_random(std::begin(buffer), std::end(buffer));
  for (std::size_t iteration = 0; iteration < ship_size; ++iteration) {
    field.field[x + (y * FIELD_SIZE)] = OCCUPIED;
    x += dx;
    y += dy;
  }
}

int
main()
{
  auto start = std::chrono::high_resolution_clock::now();
  Field field;
  std::vector<Coordinates> buffer;
  buffer.resize(FIELD_SIZE * FIELD_SIZE);
  for (const auto& ship_size: { 4, 3, 3, 2, 2, 2, 1, 1, 1, 1 }) {
    emplace_ships(field, ship_size, buffer);
  }
  auto elapsed = std::chrono::high_resolution_clock::now() - start;
  long long microseconds = std::chrono::duration_cast<std::chrono::microseconds>(elapsed).count();
  std::cout << microseconds << " ms.\n";
  std::cout << field;
}
