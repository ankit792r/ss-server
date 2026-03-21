const form = document.getElementById("form");
const messageBox = document.getElementById("messageBox");

function showMessage(message, type = "error") {
  messageBox.style.display = "block";

  if (type === "success") {
    messageBox.innerHTML = `<article class="contrast">${message}</article>`;
  } else {
    messageBox.innerHTML = `<article>${message}</article>`;
  }
}

function clearMessage() {
  messageBox.style.display = "none";
  messageBox.innerHTML = "";
}

form.addEventListener("submit", async (e) => {
  e.preventDefault();

  clearMessage();

  const objectElement = document.getElementById("objectInput");
  const keyElement = document.getElementById("keyInput");

  const object = objectElement.value.trim();
  const key = keyElement.value.trim();

  if (object && key) {
    showMessage("Please fill either Object OR Key, not both.");
    return;
  }

  if (!object && !key) {
    showMessage("Please enter something to send or a key to retrieve.");
    return;
  }

  try {
    // ✅ SEND
    if (object) {
      const data = await sendObject(object);

      if (data.success) {
        showMessage(`✅ Object stored!`);
        objectElement.value = "";
        keyElement.value = data.key;
      } else {
        showMessage("Failed to store object.");
      }
    }

    // ✅ RETRIEVE
    else if (key) {
      const data = await retrieveObject(key);

      if (data.success) {
        showMessage(`✅ Retrieved Object`);
        keyElement.value = "";
        objectElement.value = data.object;
      } else {
        showMessage(data.error || "Object not found.");
      }
    }
  } catch (err) {
    console.error(err);
    showMessage("Something went wrong. Please try again.");
  }
});

async function sendObject(object) {
  const res = await fetch("/send", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ object }),
  });

  if (!res.ok) throw new Error("Send failed");

  return await res.json();
}

async function retrieveObject(key) {
  const res = await fetch("/retrieve", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ key }),
  });

  if (!res.ok && res.status !== 404) {
    throw new Error("Retrieve failed");
  }

  return await res.json();
}
