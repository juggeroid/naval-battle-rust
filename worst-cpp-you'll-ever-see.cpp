#include <array>
#include <chrono>
#include <iomanip>
#include <iostream>
#include <random>
#include <vector>

#include <boost/container/static_vector.hpp>
#include <boost/random.hpp>

template <typename Type = std::uint64_t> class randomizer_with_sentinel_shift {
  public:
    template <typename Generator> bool operator()(Generator &rng) {
        if (1 == m_rand)
            m_rand = std::uniform_int_distribution<Type> {} (rng) | s_mask_left1;
        const bool ret = m_rand & 1;
        m_rand >>= 1;
        return ret;
    }
  private:
    static constexpr Type s_mask_left1 = Type(1) << (sizeof(Type) * 8 - 1);
    Type m_rand = 1;
};

namespace {
  
    constexpr auto FIELD_SIZE = 10;
    enum class CellType: std::uint8_t {
      EMPTY,
      UNAVAILABLE,
      OCCUPIED
    };
    using Coordinates = std::pair<std::int32_t, std::int32_t>;
    using Directions = std::array<Coordinates, 9>;

    constexpr auto DIRECTIONS = Directions {{{  0,  0 },
                                             {  0,  1 },
                                             {  0, -1 },
                                             { -1,  0 },
                                             {  1,  0 },
                                             { -1,  1 },
                                             {  1, -1 },
                                             { -1, -1 },
                                             {  1,  1 }}};

    boost::taus88 generator {std::random_device{}()};
    randomizer_with_sentinel_shift<std::uint64_t> bool_generator;
}

struct Field {
  std::array<CellType, FIELD_SIZE * FIELD_SIZE> field = {CellType::EMPTY };
  friend std::ostream& operator<<(std::ostream& o, const Field& field) {
    for (size_t index = 0; index < FIELD_SIZE * FIELD_SIZE;) {
      switch (field.field[index]) {
        case CellType::EMPTY:
          o << ".";
          break;
        case CellType::UNAVAILABLE:
          o << " ";
          break;
        case CellType::OCCUPIED:
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
is_valid_formation(const Field&       field, 
                   std::size_t        x, 
                   std::size_t        y,
                   const std::int32_t dx, 
                   const std::int32_t dy,
                   const std::size_t  ship_size) noexcept
{

  const auto within_bounds = [&](const auto x, 
                                 const auto y, 
                                 const std::pair<std::int32_t, std::int32_t>& direction = std::pair {0, 0}) noexcept {
    return ((x + direction.first  < FIELD_SIZE) && (x + direction.first  >= 0))
           && 
           ((y + direction.second < FIELD_SIZE) && (y + direction.second >= 0));
  };

  for (std::size_t iteration = 0; iteration < ship_size; ++iteration) {
    const std::int32_t nx = x + (dx * iteration);
    const std::int32_t ny = y + (dy * iteration);
    for (const auto& direction: DIRECTIONS) {
      [[unlikely]] if (!within_bounds(nx, ny, direction))
      continue;
      const auto& bounding_box_cell = field.field[(nx + direction.first) + ((ny + direction.second) * FIELD_SIZE)];
      if (CellType::OCCUPIED == bounding_box_cell) 
      return false;
    }
  }
  
  for (std::size_t iteration = 0; iteration < ship_size; ++iteration) {
    [[unlikely]] if (!within_bounds(x, y))
    return false;
    const auto& current_cell = field.field[(y * FIELD_SIZE) + x];
    if (CellType::OCCUPIED == current_cell || CellType::UNAVAILABLE == current_cell)
    return false;
    x += dx;
    y += dy;
  }
  return true;
}

void
get_available_cells(const Field&       field, 
                    const std::int32_t dx, 
                    const std::int32_t dy,
                    const std::size_t  ship_size,
                    boost::container::static_vector<Coordinates, FIELD_SIZE * FIELD_SIZE>& buffer) noexcept
{
  buffer.clear();
  for (std::size_t x = 0; x < FIELD_SIZE; ++x) {
    for (std::size_t y = 0; y < FIELD_SIZE; ++y) {
      if (is_valid_formation(field, x, y, dx, dy, ship_size))
        buffer.emplace_back(x, y);
    }
  }
}

auto emplace_ships(Field& field, 
                   const std::size_t ship_size,
                   boost::container::static_vector<Coordinates, FIELD_SIZE * FIELD_SIZE>& buffer) noexcept
{
  const auto get_alignment = [&] {
    return bool_generator(generator) 
           ? Coordinates {1, 0}
           : Coordinates {0, 1};
  };

  auto [dx, dy] = get_alignment();
  get_available_cells(field, dx, dy, ship_size, buffer);
  auto [x, y] = buffer[generator() % buffer.size()];
  for (std::size_t iteration = 0; iteration < ship_size; ++iteration) {
    field.field[x + (y * FIELD_SIZE)] = CellType::OCCUPIED;
    x += dx;
    y += dy;
  }
}

int
main()
{
  boost::container::static_vector<Coordinates, FIELD_SIZE * FIELD_SIZE> buffer;
  buffer.resize(FIELD_SIZE * FIELD_SIZE);
  Field field;
  for (auto&& ship_size: { 4, 3, 3, 2, 2, 2, 1, 1, 1, 1 }) {
    emplace_ships(field, ship_size, buffer);
  }
}
