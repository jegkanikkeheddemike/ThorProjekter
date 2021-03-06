//https://coolors.co/palette/22223b-4a4e69-9a8c98-c9ada7-f2e9e4
const port = 3000;
import express from "express";
import bodyParser from 'body-parser';
const e_app = express();

//server static files
e_app.use(express.static(__dirname + "/views"));
e_app.use(express.static(__dirname + "/static"));

//Use parsers
e_app.use(express.json());
e_app.use(bodyParser.urlencoded({     // to support URL-encoded bodies
    extended: true
}));


//Load /home
import { home } from "./server_modules/home";
home(e_app);

//load /projekter/chat_app
import { chat_app } from "./server_modules/chat_app";
chat_app(e_app);

//When visited on /, redirect to /home
e_app.get('/', (req, res) => {
    res.redirect("/home");
});

e_app.listen(port, () => {
    console.log(`    Server listening at on ${port}`)
})