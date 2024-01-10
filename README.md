# togetherness

Приложение ко второму празднику Единения 2024 в Политехе.

## Цель

Мы хотим позволить гостям Праздника «отравить», «похитить», «наградить» друг друга. При этом избежать коллизий, накруток и облегчить подсчёт — что было бы трудно сделать, используя «тетради смерти» или жетоны.

## Запуск

Тестировалось на Ubuntu 22.04 с установленными docker и docker-compose.

Находясь в директории `deploy`, выполнить `sudo docker-compose up`. Фронтенд будет слушать на `0.0.0.0:8079`, бэкенд на `0.0.0.0:8080`, pgadmin на `0.0.0.0:8081`. Внешнего доступа к БД нет.


## Предпросмотр фронта

https://659ebaeb272ad10800eaf3ed--tourmaline-gnome-44c32e.netlify.app/

Задел под мобильные экраны. Адаптивной верстки нет.
