import type { Product, ProductRecord } from "model";

// Helper functions
const randomInt = (min: number, max: number): number =>
  Math.floor(Math.random() * (max - min + 1)) + min;

const randomBool = (): boolean => Math.random() < 0.5;

const randomDate = (start: Date, end: Date): Date =>
  new Date(start.getTime() + Math.random() * (end.getTime() - start.getTime()));

const randomDescription = (): string => {
  const adjectives = [
    'Fresh',
    'Organic',
    'Premium',
    'Delicious',
    'Exotic',
    'Handmade',
    'Authentic',
    'Traditional',
    'Natural',
    'Classic'
  ];
  const nouns = [
    'Tofu',
    'Tempeh',
    'Soy Milk',
    'Pudding',
    'Snack',
    'Meal',
    'Product',
    'Dessert',
    'Dish',
    'Delicacy'
  ];
  return `${adjectives[randomInt(0, adjectives.length - 1)]} ${nouns[randomInt(0, nouns.length - 1)]}`;
};

// Generate Products
export const generateMockProduct = (count: number = 50): ProductRecord => {
  const products: ProductRecord = [];

  for (let i = 1; i <= count; i++) {
    const product: Product = {
      id: i,
      userId: randomInt(1, 10),
      paid: randomBool(),
      productionDate: randomDate(new Date(2024, 0, 1), new Date()),
      takenDate: randomDate(new Date(2024, 0, 1), new Date()),
      price: parseFloat((Math.random() * 100).toFixed(2)),
      amount: randomInt(1, 20),
      description: randomDescription(),
    };

    products.push(product);
  }

  return products;
};

