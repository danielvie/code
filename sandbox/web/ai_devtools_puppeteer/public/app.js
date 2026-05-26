const price = 3;
let quantity = 0;
let addDelay = 0;

const addButton = document.querySelector("#add-coffee");
const quantityText = document.querySelector("#quantity");
const totalText = document.querySelector("#total");
const statusText = document.querySelector("#status");

function renderCart() {
  quantityText.textContent = String(quantity);

  const total = price * quantity;

  totalText.textContent = `$${total}`;
  statusText.textContent = `Cart has ${quantity} coffee${quantity === 1 ? "" : "s"}.`;
}

addButton.addEventListener("click", () => {
  addDelay += 300;
  addButton.textContent = "Adding...";

  setTimeout(() => {
    quantity += 1;
    renderCart();
    addButton.textContent = "Add coffee";
  }, addDelay);
});
