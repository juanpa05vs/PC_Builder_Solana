# 🖥️ PC Builder Solana

Sistema de armado de computadoras desarrollado como **Solana Program** utilizando **Rust** y el framework **Anchor**.  

Este proyecto implementa un sistema **CRUD** para gestionar componentes de una PC dentro de un carrito en blockchain, aplicando:

- 🔑 Program Derived Addresses (PDAs)  
- ⚡ Optimización de memoria *On-Chain*  
- 🔒 Seguridad basada en firmas  

---

## 📚 Descripción

**PC Builder Solana** simula un configurador de computadoras donde cada usuario puede:

- Crear un proyecto de armado de PC  
- Agregar componentes (CPU, GPU, RAM, etc.)  
- Editar especificaciones y precios  
- Eliminar componentes del carrito  
- Consultar su build completa en blockchain  

---

## 🧠 Arquitectura y Estructuras de Datos

En Solana es necesario definir el tamaño de los datos para calcular correctamente la renta (*rent*).

### 📦 PDA Principal: `CarritoPC`

Cuenta raíz que almacena el carrito de componentes.

```rust
#[account]
#[derive(InitSpace)]
pub struct CarritoPC {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_proyecto: String,
    #[max_len(12)]
    pub componentes: Vec<Componente>,
}
```

---

### 🧩 Estructura Interna: `Componente`

Cada componente contiene:

- `categoria (String)` → tipo de componente (CPU, GPU, etc.)  
- `modelo (String)` → nombre del modelo  
- `precio_estimado (u32)` → costo estimado  

```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Componente {
    #[max_len(20)]
    pub categoria: String,
    #[max_len(30)]
    pub modelo: String,
    pub precio_estimado: u32,
}
```

---

## 🔒 Seguridad

El contrato asegura que solo el dueño pueda modificar el carrito:

```rust
require!(
    carrito.owner == ctx.accounts.owner.key(),
    Errores::NoEresElDueno
);
```

✔ Protege el acceso al carrito  
✔ Evita modificaciones por terceros  

---

## ⚙️ Funcionalidad (CRUD)

### 🟢 Inicializar Carrito

Crea la cuenta principal usando:

```rust
[b"pcbuild", owner.key().as_ref()]
```

Inicializa:
- Owner  
- Nombre del proyecto  
- Lista vacía de componentes  

---

### ➕ Agregar Componente

- Recibe:
  - categoría  
  - modelo  
  - precio  
- Inserta en el vector con `.push()`  

---

### ✏️ Editar Componente

- Busca por `modelo`  
- Actualiza:
  - categoría  
  - precio  

---

### ❌ Eliminar Componente

```rust
.iter().position(|c| c.modelo == modelo)
```

- Si existe → `.remove(index)`  
- Si no → error `ComponenteNoEncontrado`  

---

### 📖 Ver Carrito

```rust
msg!("Componentes en Carrito: {:#?}", carrito.componentes);
```

Muestra todos los componentes en logs *On-Chain*

---

## 🧪 Despliegue en Solana Playground

1. Copia el código en `lib.rs`  
2. Ejecuta:

```bash
cargo clean
```

3. Haz clic en **Build**  
4. Haz clic en **Deploy (Devnet)**  

---

## 🧑‍💻 Pruebas

Puedes interactuar con el contrato usando:

- Pestaña **Test** del Playground  
- Scripts en TypeScript:

```ts
pg.program.methods...
```

Parámetros:
- `categoria: String`  
- `modelo: String`  
- `precio: u32`  

---

## ⚠️ Manejo de Errores

```rust
#[error_code]
pub enum Errores {
    #[msg("Acceso denegado: No eres el dueño de este carrito.")]
    NoEresElDueno,
    #[msg("El componente no se encuentra en el carrito.")]
    ComponenteNoEncontrado,
}
```

---

## 📌 Conclusión

Este proyecto demuestra:

- Gestión de datos estructurados en Solana  
- Seguridad mediante validación de firmas  
- Uso eficiente de vectores dinámicos  
- Aplicación de CRUD en un caso práctico (PC Builder)  

---

## 🚀 Próximos pasos

- Integrar frontend (React / Next.js)  
- Calcular precio total automáticamente  
- Añadir compatibilidad entre componentes  
- Integrar tiendas reales (APIs externas)  

---
