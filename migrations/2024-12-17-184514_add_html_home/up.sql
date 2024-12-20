-- Dodanie rekordów do tabeli HTML
INSERT INTO "HTML" (name, element) VALUES
('logged_out_Header', '  <header>
  <h1 class="logo">
      Roommates Organiser
  </h1>
    <nav>
      <ul class="menu">
        <li><a href="/logged_out/home" class="genericButton">Home</a></li>
        <li><a href="/logged_out/support" class="genericButton">Support</a></li> 
        <li><a href="/logged_out/register" class="genericButton">Register</a></li>
        <li><a href="/logged_out/log_in" class="genericButton">Log in</a></li>
      </ul>
    </nav>
  </header>'),
('Footer', '  <footer>
    <p>&copy; 2024 Roommate Organizer. All rights reserved.</p>
  </footer>'),
('first_lines', '<!DOCTYPE html>
<html lang="en">'),

-- LOGGEDOUT INDEX.HTML
('logged_out_index_head', '<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&display=swap" rel="stylesheet">
  <title>Roommates Organiser</title>
  <link rel="stylesheet" href="/static/css/base-styles.css">
  <link rel="stylesheet" href="/static/css/logo.css">
  <link rel="stylesheet" href="/static/css/genericButton.css">
  <link rel="stylesheet" href="/static/css/genericContent.css">
  <link rel="stylesheet" href="/static/css/footer.css">
  <link rel="icon" type="image/x-icon" href="/static/favicon.ico">
</head>'),
('logged_out_index_body_individual', '<main>
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
('logged_out_support_body_individual',  '<main>
    <section id="home" class="genericContent">
      <h2>Support</h2>
      <p>If you need assistance, please send an email to <strong>support@roommateorganiser.com</strong>. Our team is here to help you with any questions or issues you may have.</p>
      <p>We look forward to assisting you!</p>
    </section>
  </main>' ),
('logged_out_register_body_individual', '<main>
    <section id="register" class="authFromContent">
    <h2>Register</h2>
    <p>Please fill out the form below to create a new account.</p>
      
    <form class="authForm" id="registrationForm">
      <div class="form-group">
        <label for="first-name">First Name</label>
        <input type="text" id="first-name" name="first_name" required>
      </div>
    
      <div class="form-group">
        <label for="last-name">Last Name</label>
        <input type="text" id="last-name" name="last_name" required>
      </div>
    
      <div class="form-group">
        <label for="email">Email Address</label>
        <input type="email" id="email" name="email" required>
      </div>
    
      <div class="form-group">
        <label for="username">User name</label>
        <input type="text" id="username-register" name="username" required>
      </div>

      <div class="form-group">
        <label for="password">Password</label>
        <input type="password" id="password-register" name="password" required minlength="6">
      </div>
          
      <button class="genericButton" type="submit">Create Account</button>
    </form>
  </section>
  </main>'),
  ('logged_out_form_head', '<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Roommates Organiser</title>
  <link rel="stylesheet" href="/static/css/base-styles.css">
  <link rel="stylesheet" href="/static/css/logo.css">
  <link rel="stylesheet" href="/static/css/authForm.css">
  <link rel="stylesheet" href="/static/css/genericContent.css">
  <link rel="stylesheet" href="/static/css/genericButton.css">
  <link rel="stylesheet" href="/static/css/footer.css">
  <link rel="icon" type="image/x-icon" href="/static/favicon.ico">
</head>'),
('logged_out_register_script','<script type="module" src="/static/scripts/Auth/RegisterForm.js"></script>'),
('logged_out_login_body_individual','<main>
    <section id="login" class="authFromContent">
    <h2>Log In</h2>
    <p>Enter your credentials to log in.</p>        
      <form class="authForm" id="logInForm" >
        <div class="form-group">
          <label for="username">Username</label>
          <input type="text" id="username-login" name="username" required>
        </div>
        <div class="form-group">
          <label for="password">Password</label>
          <input type="password" id="password-login" name="password" required>
        </div>            
        <button class="genericButton" type="submitLogIn">Log in</button>
      </form>
    </main>'),
('logged_out_login_script', '<script type="module" src="/static/scripts/Auth/SubmitLogInForm.js"></script>');