-- Your SQL goes here
-- Dodanie rekordów do tabeli HTML
INSERT INTO "HTML" (name, element) VALUES
('logged_in_basic_head', 
'<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&display=swap" rel="stylesheet">
  <title>Roommates Organiser</title>
  <link rel="stylesheet" href="/static/css/base-styles.css">
  <link rel="stylesheet" href="/static/css/logo.css">
  <link rel="stylesheet" href="/static/css/genericButton.css">
  <link rel="stylesheet" href="/static/css/genericContent.css">
  <link rel="stylesheet" href="/static/css/drop-down.css">
  <link rel="stylesheet" href="/static/css/footer.css">
  <link rel="icon" type="image/x-icon" href="/static/favicon.ico">
</head>'),
('logged_in_header',
'  <header>
  <h1 class="logo">
    Roommates Organiser
  </h1>
    <nav>
      <ul class="menu">
        <li><a href="/logged_in/home" class="genericButton">Home</a></li>
        <li id=groupsDropDown>
          <div class="dropdown">
            <button class="genericButton">Groups</button>
            <div class="dropdown-content">
              <a href="/logged_in/my_groups">my groups</a>
              <a href="/logged_in/manage_owned_groups">Manage owned groups</a>
              <a href="/logged_in/join_group">Join group</a>
              <a href="/logged_in/create_group">Create group</a>
            </div>
          </div>
        </li>
        <li><a href="/logged_in/support" class="genericButton">Support</a></li>
        <li><button id = "logoutbutton" class="genericButton">Log out</button></li>
      </ul>
    </nav>
  </header>'),
('logged_in_home_main', 
'<main>
    <section id="home" class="genericContent">
      <h2>Welcome to Roommate Organiser!</h2>
      <p>
          Managing shared expenses has never been easier. Roommate Organiser is your ultimate tool for 
          tracking expenses, splitting bills, and keeping your household finances transparent and stress-free.
      </p>
      <p>
          Whether you''re sharing an apartment with friends or living with family, Roommate Organiser ensures 
          everyone contributes fairly. Say goodbye to awkward money conversations and hello to hassle-free 
          expense management!
      </p>
      <ul>
          <li>Track shared expenses in real time</li>
          <li>Split bills automatically and fairly</li>
          <li>Keep a clear overview of who owes what</li>
      </ul>
      <p>
          Join today and take the first step toward organized and stress-free living!
      </p>
    </section>
</main>'),
('logged_in_home_scripts', 
'<script src="/static/scripts/Auth/logOut.js"></script>
<script src="/static/scripts/Auth/session_checker.js"></script>');