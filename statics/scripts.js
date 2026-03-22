const form = document.getElementById("form");
const messageBox = document.getElementById("messageBox");

const objectElement = document.getElementById("objectInput");
const keyElement = document.getElementById("keyInput");

const copyObjectBtn = document.getElementById("copyObjectBtn");
const copyKeyBtn = document.getElementById("copyKeyBtn");

function showMessage(message) {
  messageBox.style.display = "block";
  messageBox.innerHTML = `<article class="contrast">${message}</article>`;
}

function clearMessage() {
  messageBox.style.display = "none";
  messageBox.innerHTML = "";
}

form.addEventListener("submit", async (e) => {
  e.preventDefault();

  clearMessage();

  const object = objectElement.value.trim();
  const key = keyElement.value.trim();

  if (object && key) {
    showMessage("❌ Please fill either Object OR Key, not both.");
    objectElement.style.borderColor = "";
    keyElement.style.borderColor = "";
    return;
  }

  if (!object && !key) {
    showMessage("❌ Please enter something to send or a key to retrieve.");
    objectElement.style.borderColor = "";
    keyElement.style.borderColor = "";
    return;
  }

  try {
    if (object) {
      const data = await sendObject(object);

      if (data.success) {
        showMessage(`✅ Object stored!`);
        objectElement.value = "";
        objectElement.style.borderColor = "";
        keyElement.value = data.key;
        keyElement.style.borderColor = "#058686";
        copyKeyBtn.style.display = "block";
        copyObjectBtn.style.display = "none";
      } else {
        showMessage("❌ Failed to store object.");
        objectElement.style.borderColor = "#D93526";
      }
    } else if (key) {
      const data = await retrieveObject(key);

      if (data.success) {
        showMessage(`✅ Retrieved Object`);
        keyElement.value = "";
        keyElement.style.borderColor = "";
        objectElement.value = data.object;
        objectElement.style.borderColor = "#058686";
        copyObjectBtn.style.display = "block";
        copyKeyBtn.style.display = "none";
      } else {
        showMessage(data.error || "❌ Object not found.");
        keyElement.style.borderColor = "#D93526";
      }
    }
  } catch (err) {
    console.error(err);
    showMessage("❌ Something went wrong. Please try again.");
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

copyObjectBtn.addEventListener("click", async () => {
  const value = objectElement.value;
  if (!value) return;

  await navigator.clipboard.writeText(value);

  copyObjectBtn.textContent = "Copied";
  setTimeout(() => {
    copyObjectBtn.textContent = "📋 Copy";
  }, 1500);
});

copyKeyBtn.addEventListener("click", async () => {
  const value = keyElement.value;
  if (!value) return;

  await navigator.clipboard.writeText(value);
  copyKeyBtn.textContent = "Copied";

  setTimeout(() => {
    copyKeyBtn.textContent = "📋 Copy";
  }, 1500);
});
