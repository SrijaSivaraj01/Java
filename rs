<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
  <title>My First Web Page</title>
  <link rel="stylesheet" href="styles.css" />
</head>
<style>
body {
  font-family: Arial, sans-serif;
  margin: 0;
  padding: 0;
  background-color: #f4f4f4;
}

header {
  background-color: #333;
  color: white;
  padding: 1em;
  text-align: center;
}

nav a {
  color: white;
  margin: 0 15px;
  text-decoration: none;
}

main {
  padding: 20px;
}

footer {
  background-color: #333;
  color: white;
  text-align: center;
  padding: 10px;
  position: fixed;
  width: 100%;
  bottom: 0;
}
</style>
<body>
  <header>
    <h1>Welcome to My Website</h1>
    <nav>
      <a href="#about">About</a>
      <a href="#contact">Contact</a>
    </nav>
  </header>

  <main>
    <section id="about">
      <h2>About Me</h2>
      <p>This is a simple paragraph about myself.</p>
    </section>

    <section id="contact">
      <h2>Contact</h2>
      <button onclick="showMessage()">Click Me</button>
    </section>
  </main>

  <footer>
    <p>&copy; 2025 My Website</p>
  </footer>

  <script src="script.js"></script>
</body>
</html>
