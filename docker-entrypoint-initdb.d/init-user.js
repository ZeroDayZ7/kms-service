// docker-entrypoint-initdb.d/init-user.js

// Odczytujemy zmienne środowiskowe przekazane do kontenera przez Docker Compose
const appUser = process.env.MONGO_APP_USER;
const appPwd = process.env.MONGO_APP_PASSWORD;
const appDb = process.env.MONGO_APP_DATABASE;

// Przełączamy się na bazę aplikacji
db = db.getSiblingDB(appDb);

// Tworzymy dedykowanego użytkownika aplikacji
db.createUser({
  user: appUser,
  pwd: appPwd,
  roles: [
    { role: "readWrite", db: appDb },
    { role: "dbAdmin", db: appDb },
  ],
});
