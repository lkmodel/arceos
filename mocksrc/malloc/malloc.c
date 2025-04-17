#include <malloc.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Define a simple structure
typedef struct Student {
    int id;
    char *name;
    float score;
} Student;

// Function declarations
void test_simple_malloc();
void test_array_malloc();
void test_struct_malloc();
void test_2d_array_malloc();

void test_realloc();
void test_calloc();
void test_realloc_string();
void test_calloc_struct();

// Structure for testing
typedef struct {
    int id;
    double value;
} DataItem;

int main()
{
    printf("=== Testing Basic Memory Allocation ===\n");
    test_simple_malloc();

    printf("\n=== Testing Array Memory Allocation ===\n");
    test_array_malloc();

    printf("\n=== Testing Structure Memory Allocation ===\n");
    test_struct_malloc();

    printf("\n=== Testing 2D Array Memory Allocation ===\n");
    test_2d_array_malloc();

    printf("=== Testing realloc ===\n");
    test_realloc();

    printf("\n=== Testing calloc ===\n");
    test_calloc();

    printf("\n=== Testing realloc with string ===\n");
    test_realloc_string();

    printf("\n=== Testing calloc with struct array ===\n");
    test_calloc_struct();

    return 0;
}

// Test basic memory allocation
void test_simple_malloc()
{
    int *ptr = (int *)malloc(sizeof(int));
    if (ptr == NULL) {
        printf("Memory allocation failed\n");
        return;
    }

    *ptr = 42;
    printf("Allocated integer value: %d\n", *ptr);

    free(ptr);
    ptr = NULL; // Avoid dangling pointer
}

// Test array memory allocation
void test_array_malloc()
{
    int size = 5;
    int *arr = (int *)malloc(size * sizeof(int));
    if (arr == NULL) {
        printf("Array memory allocation failed\n");
        return;
    }

    // Initialize array
    for (int i = 0; i < size; i++) {
        arr[i] = i * 10;
    }

    // Print array
    printf("Array elements: ");
    for (int i = 0; i < size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    free(arr);
    arr = NULL;
}

// Test structure memory allocation
void test_struct_malloc()
{
    Student *student = (Student *)malloc(sizeof(Student));
    if (student == NULL) {
        printf("Structure memory allocation failed\n");
        return;
    }

    // Allocate memory for structure string member
    student->name = (char *)malloc(20 * sizeof(char));
    if (student->name == NULL) {
        free(student);
        printf("String memory allocation failed\n");
        return;
    }

    // Initialize structure
    student->id = 1001;
    strcpy(student->name, "John");
    student->score = 85.5;

    printf("Student Information:\n");
    printf("ID: %d\n", student->id);
    printf("Name: %s\n", student->name);
    printf("Score: %.1f\n", student->score);

    // Note the order of freeing: first free internal allocations
    free(student->name);
    free(student);
    student = NULL;
}

// Test 2D array memory allocation
void test_2d_array_malloc()
{
    int rows = 3;
    int cols = 4;

    // Allocate row pointer array
    int **matrix = (int **)malloc(rows * sizeof(int *));
    if (matrix == NULL) {
        printf("Matrix memory allocation failed\n");
        return;
    }

    // Allocate memory for each row
    for (int i = 0; i < rows; i++) {
        matrix[i] = (int *)malloc(cols * sizeof(int));
        if (matrix[i] == NULL) {
            // Free already allocated memory
            for (int j = 0; j < i; j++) {
                free(matrix[j]);
            }
            free(matrix);
            printf("Matrix row memory allocation failed\n");
            return;
        }
    }

    // Initialize matrix
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            matrix[i][j] = i * cols + j;
        }
    }

    // Print matrix
    printf("Matrix contents:\n");
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            printf("%2d ", matrix[i][j]);
        }
        printf("\n");
    }

    // Free matrix memory
    for (int i = 0; i < rows; i++) {
        free(matrix[i]);
    }
    free(matrix);
    matrix = NULL;
}

// Demonstrate realloc for array resizing
void test_realloc()
{
    int *numbers;
    int initial_size = 3;
    int new_size = 5;

    // Initial allocation
    numbers = (int *)malloc(initial_size * sizeof(int));
    if (numbers == NULL) {
        printf("Initial allocation failed\n");
        return;
    }

    // Initialize initial array
    for (int i = 0; i < initial_size; i++) {
        numbers[i] = i + 1;
    }

    // Print initial array
    printf("Initial array: ");
    for (int i = 0; i < initial_size; i++) {
        printf("%d ", numbers[i]);
    }
    printf("\n");

    // Reallocate to larger size
    int *temp = (int *)realloc(numbers, new_size * sizeof(int));
    if (temp == NULL) {
        printf("Reallocation failed\n");
        free(numbers);
        return;
    }
    numbers = temp;

    // Add new elements
    for (int i = initial_size; i < new_size; i++) {
        numbers[i] = i + 1;
    }

    // Print expanded array
    printf("Expanded array: ");
    for (int i = 0; i < new_size; i++) {
        printf("%d ", numbers[i]);
    }
    printf("\n");

    free(numbers);
}

// Demonstrate calloc for zero-initialized array
void test_calloc()
{
    int size = 5;
    int *numbers = (int *)calloc(size, sizeof(int));
    if (numbers == NULL) {
        printf("Calloc allocation failed\n");
        return;
    }

    // Print zero-initialized array
    printf("Zero-initialized array: ");
    for (int i = 0; i < size; i++) {
        printf("%d ", numbers[i]);
    }
    printf("\n");

    // Modify some values
    numbers[1] = 10;
    numbers[3] = 20;

    // Print modified array
    printf("Modified array: ");
    for (int i = 0; i < size; i++) {
        printf("%d ", numbers[i]);
    }
    printf("\n");

    free(numbers);
}

// Demonstrate realloc with string
void test_realloc_string()
{
    char *str = (char *)malloc(10 * sizeof(char));
    if (str == NULL) {
        printf("Initial string allocation failed\n");
        return;
    }

    strcpy(str, "Hello");
    printf("Initial string: %s\n", str);

    // Reallocate for a longer string
    char *temp = (char *)realloc(str, 20 * sizeof(char));
    if (temp == NULL) {
        printf("String reallocation failed\n");
        free(str);
        return;
    }
    str = temp;

    strcat(str, " World!");
    printf("Extended string: %s\n", str);

    free(str);
}

// Demonstrate calloc with structure array
void test_calloc_struct()
{
    int count = 3;

    // Allocate array of structures
    DataItem *items = (DataItem *)calloc(count, sizeof(DataItem));
    if (items == NULL) {
        printf("Structure array allocation failed\n");
        return;
    }

    // Print initial values (should be zero)
    printf("Initial structure values:\n");
    for (int i = 0; i < count; i++) {
        printf("Item %d: id = %d, value = %.2f\n", i, items[i].id, items[i].value);
    }

    // Set some values
    items[0].id = 1;
    items[0].value = 10.5;
    items[1].id = 2;
    items[1].value = 20.7;

    // Print modified values
    printf("\nModified structure values:\n");
    for (int i = 0; i < count; i++) {
        printf("Item %d: id = %d, value = %.2f\n", i, items[i].id, items[i].value);
    }

    free(items);
}
