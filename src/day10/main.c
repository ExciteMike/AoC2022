#include <stdio.h>
#include <string.h>
#include <ctype.h>

int xs[256];

void get_xs()
{
    char buf[1024] = {0};
    FILE *f = fopen("../../puzzle_input/day10", "r");
    fread(buf, sizeof(char), sizeof(buf) / sizeof(char), f);
    fclose(f);

    char *read_head = buf;
    int *write_head = xs;
    *(write_head++) = 1;
    *(write_head++) = 1;
    while (1)
    {
        *write_head = *(write_head - 1);
        ++write_head;
        char *line_end = strchr(read_head, '\n');
        if (line_end == NULL)
        {
            break;
        }
        if (read_head[0] == 'a')
        {
            read_head += 5;
            int isneg = *read_head == '-';
            if (isneg)
            {
                ++read_head;
            }
            int v = 0;
            while (isdigit(*read_head))
            {
                v = 10 * v + (*read_head - '0');
                ++read_head;
            }
            if (isneg)
            {
                v = -v;
            }
            *write_head = *(write_head - 1) + v;
            ++write_head;
        }

        read_head = line_end + 1;
    }
}

void p1()
{
    int sum = 0;
    for (int i = 20; i < 221; i += 40)
    {
        sum += i * xs[i];
    }

    printf("part 1: %d\n", sum);
}

int p2()
{
    printf("part 2: \n");
    for (int y = 0; y < 6; ++y)
    {
        for (int x = 0; x < 40; ++x)
        {
            int delta = x - xs[y * 40 + x];
            if ((0 <= delta) && (delta < 3))
            {
                printf("[]");
            }
            else
            {
                printf("  ");
            };
        }
        printf("\n");
    }
}

int main()
{
    get_xs();
    p1();
    p2();
}