# togetherness

Приложение ко второму празднику Единения 2024 в Политехе.

## Цель

Мы хотим позволить гостям Праздника «отравить», «похитить», «наградить» друг друга. При этом избежать коллизий, накруток и облегчить подсчёт — что было бы трудно сделать, используя «тетради смерти» или жетоны.

## Запуск бэкенда и БД

Тестировалось на Ubuntu 22.04 с установленными docker и docker-compose.

Находясь в директории `deploy`, выполнить `sudo docker-compose up`. Бэкенд будет слушать на `0.0.0.0:8080`, pgadmin на `0.0.0.0:8081`. Внешнего доступа к БД нет.
