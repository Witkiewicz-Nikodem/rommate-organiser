-- Dodanie rekordów do tabeli HTML
INSERT INTO "HTML" (name, element) VALUES
('logged_in_create_group_individual',
'<main>
    <section id="createGroup" class="content genericContent">
      <h2>Create group</h2>
      <p>Provide the name of the new group.</p>        
        <form class="authForm" id="CreateGroupForm">      
          <div class="form-group">
            <label for="name">name</label>
            <input type="text" id="name-create-group" name="name" required>
          </div>            
          <button class="genericButton" type="submit">Create new Group</button>
        </form>            
  </section>
 </main>
'),
('logged_in_create_group_scripts',
'<script src="/static/scripts/Auth/logOut.js"></script>
<script src="/static/scripts/CRUD/create_group.js"></script>
'),
('logged_in_create_group_head',
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


  