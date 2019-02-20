#include <stddef.h>

int adjacentElementsProduct(int inputArray[], size_t input_size) 
{
  int max_product = inputArray[0] * inputArray[1];
  
  for (size_t i = 1; i < input_size - 1; i++)
  {
    if (max_product < inputArray[i] * inputArray[i + 1]) 
    {
        max_product = inputArray[i] * inputArray[i + 1];
    }
  }

  return max_product;
}