-- Dodanie rekordów do tabeli HTML
INSERT INTO "HTML" (name, element) VALUES
('logged_in_join_group_individual',
'<main>
    <section id="joinGroup" class="content genericContent">
      <h2>Join group</h2>
      <p>Provide code sent by group owner.</p>        
        <form class="authForm" id="JoinGroupForm">      
          <div class="form-group">
            <label for="code">code</label>
            <input type="text" id="name-create-group" name="code" required>
          </div>            
          <button class="genericButton" type="submit">Join group</button>
        </form>                
    </section>
 </main>
'),
('logged_in_join_group_scripts',
'<script src="/static/scripts/Auth/logOut.js"></script>
<script src="/static/scripts/CRUD/join_group.js"></script>
<script src="/static/scripts/Auth/session_checker.js"></script>
'),
('logged_in_join_group_head',
'<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&display=swap" rel="stylesheet">
  <title>Roommates Organiser</title>
  <link rel="stylesheet" href="/static/css/base-styles.css">
  <link rel="stylesheet" href="/static/css/logo.css">
  <link rel="stylesheet" href="/static/css/genericButton.css">
  <link rel="stylesheet" href="/static/css/genericContent.css">
  <link rel="stylesheet" href="/static/css/footer.css">
  <link rel="stylesheet" href="/static/css/authForm.css">
  <link rel="stylesheet" href="/static/css/drop-down.css">
  <link rel="icon" type="image/x-icon" href="/static/favicon.ico">
</head>
')


  