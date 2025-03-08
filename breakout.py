import pygame
import random

# Initialize Pygame
pygame.init()

# Screen dimensions
SCREEN_WIDTH = 800
SCREEN_HEIGHT = 600

# Colors
WHITE = (255, 255, 255)
BLACK = (0, 0, 0)
RED = (255, 0, 0)
GREEN = (0, 255, 0)
BLUE = (0, 0, 255)

# Ball class
class Ball:
    def __init__(self, x, y, radius, color):
        self.x = x
        self.y = y
        self.radius = radius
        self.color = color
        self.dx = random.choice([-4, 4])
        self.dy = -4

    def move(self):
        self.x += self.dx
        self.y += self.dy

    def draw(self, screen):
        pygame.draw.circle(screen, self.color, (self.x, self.y), self.radius)

    def check_collision(self, paddle, bricks):
        if self.x - self.radius <= 0 or self.x + self.radius >= SCREEN_WIDTH:
            self.dx = -self.dx
        if self.y - self.radius <= 0:
            self.dy = -self.dy
        if self.y + self.radius >= SCREEN_HEIGHT:
            return True

        if paddle.rect.collidepoint(self.x, self.y + self.radius):
            self.dy = -self.dy

        for brick in bricks:
            if brick.rect.collidepoint(self.x, self.y - self.radius):
                bricks.remove(brick)
                self.dy = -self.dy
                break

        return False

# Paddle class
class Paddle:
    def __init__(self, x, y, width, height, color):
        self.rect = pygame.Rect(x, y, width, height)
        self.color = color
        self.dx = 0

    def move(self):
        self.rect.x += self.dx
        if self.rect.left < 0:
            self.rect.left = 0
        if self.rect.right > SCREEN_WIDTH:
            self.rect.right = SCREEN_WIDTH

    def draw(self, screen):
        pygame.draw.rect(screen, self.color, self.rect)

# Brick class
class Brick:
    def __init__(self, x, y, width, height, color):
        self.rect = pygame.Rect(x, y, width, height)
        self.color = color

    def draw(self, screen):
        pygame.draw.rect(screen, self.color, self.rect)

# Main game loop
def main():
    screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
    pygame.display.set_caption("Breakout Clone")

    clock = pygame.time.Clock()

    ball = Ball(SCREEN_WIDTH // 2, SCREEN_HEIGHT // 2, 10, WHITE)
    paddle = Paddle(SCREEN_WIDTH // 2 - 50, SCREEN_HEIGHT - 30, 100, 10, WHITE)
    bricks = [Brick(x * 60 + 10, y * 30 + 10, 50, 20, random.choice([RED, GREEN, BLUE])) for x in range(12) for y in range(5)]

    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_LEFT:
                    paddle.dx = -6
                elif event.key == pygame.K_RIGHT:
                    paddle.dx = 6
            elif event.type == pygame.KEYUP:
                if event.key == pygame.K_LEFT or event.key == pygame.K_RIGHT:
                    paddle.dx = 0

        ball.move()
        paddle.move()

        if ball.check_collision(paddle, bricks):
            running = False

        screen.fill(BLACK)
        ball.draw(screen)
        paddle.draw(screen)
        for brick in bricks:
            brick.draw(screen)

        pygame.display.flip()
        clock.tick(60)

    pygame.quit()

if __name__ == "__main__":
    main()
