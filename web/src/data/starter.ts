// The items the app comes with, to pick from before the first shop. This is data, not
// rules: change it freely. An item's id comes from its name, so renaming an entry here
// starts it afresh, while moving it to another aisle keeps what the user did with it.
import type { StarterItem } from '@/core'

/** Where items the user adds themselves are shown. */
export const MY_ITEMS = 'My items'

const aisles: Record<string, string[]> = {
  'Fruit & veg': ['Bananas', 'Apples', 'Tomatoes', 'Cucumbers', 'Onions', 'Avocados', 'Lettuce'],
  'Dairy & eggs': ['Milk', 'Eggs', 'Yogurt', 'Cottage cheese', 'Butter'],
  Bread: ['Bread', 'Pitas', 'Rice cakes'],
  Pantry: ['Rice', 'Pasta', 'Olive oil', 'Tuna', 'Coffee', 'Tea'],
  Snacks: ['Crackers', 'Chocolate', 'Nuts'],
  Home: ['Toilet paper', 'Dish soap', 'Trash bags'],
}

export const starterItems: StarterItem[] = Object.entries(aisles).flatMap(([aisle, names]) =>
  names.map((name) => ({ id: `starter:${name.toLowerCase()}`, name, aisle })),
)
