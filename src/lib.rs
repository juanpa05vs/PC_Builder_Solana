use anchor_lang::prelude::*;

// ID del programa (Se genera al hacer build en SolPG)
declare_id!("FFqGmX8zsZu65GQC7SeidN9toTLhtV91kWLiBwB3f5aw");

#[program]
pub mod pc_builder_solana {
    use super::*;

    // 1. CREATE (PDA): Inicializa el carrito de componentes
    pub fn inicializar_carrito(ctx: Context<CrearCarrito>, nombre_proyecto: String) -> Result<()> {
        let carrito = &mut ctx.accounts.carrito;
        carrito.owner = ctx.accounts.owner.key();
        carrito.nombre_proyecto = nombre_proyecto;
        carrito.componentes = Vec::new();
        
        msg!("Carrito de PC '{}' inicializado.", carrito.nombre_proyecto);
        Ok(())
    }

    // 2. CREATE (Dato): Agrega un componente exigiendo categoría, modelo y precio
    pub fn agregar_componente(
        ctx: Context<GestionarCarrito>, 
        categoria: String, 
        modelo: String, 
        precio: u32
    ) -> Result<()> {
        let carrito = &mut ctx.accounts.carrito;
        require!(carrito.owner == ctx.accounts.owner.key(), Errores::NoEresElDueno);

        let nuevo_componente = Componente {
            categoria,
            modelo,
            precio_estimado: precio,
        };

        carrito.componentes.push(nuevo_componente);
        msg!("Componente agregado al presupuesto.");
        Ok(())
    }

    // 3. UPDATE: Modifica los detalles de un componente existente (Búsqueda por modelo)
    pub fn editar_componente(
        ctx: Context<GestionarCarrito>, 
        modelo: String, 
        nueva_categoria: String, 
        nuevo_precio: u32
    ) -> Result<()> {
        let carrito = &mut ctx.accounts.carrito;
        require!(carrito.owner == ctx.accounts.owner.key(), Errores::NoEresElDueno);

        let lista = &mut carrito.componentes;
        for i in 0..lista.len() {
            if lista[i].modelo == modelo {
                lista[i].categoria = nueva_categoria;
                lista[i].precio_estimado = nuevo_precio;
                msg!("Datos del componente '{}' actualizados.", modelo);
                return Ok(());
            }
        }
        Err(Errores::ComponenteNoEncontrado.into())
    }

    // 4. DELETE: Elimina un componente del carrito
    pub fn eliminar_componente(ctx: Context<GestionarCarrito>, modelo: String) -> Result<()> {
        let carrito = &mut ctx.accounts.carrito;
        require!(carrito.owner == ctx.accounts.owner.key(), Errores::NoEresElDueno);

        let lista = &mut carrito.componentes;
        let index = lista.iter().position(|c| c.modelo == modelo);

        if let Some(i) = index {
            lista.remove(i);
            msg!("Componente '{}' eliminado del carrito.", modelo);
            Ok(())
        } else {
            Err(Errores::ComponenteNoEncontrado.into())
        }
    }

    // 5. READ: Visualiza el carrito completo
    pub fn ver_carrito(ctx: Context<GestionarCarrito>) -> Result<()> {
        msg!("Proyecto: {}", ctx.accounts.carrito.nombre_proyecto);
        msg!("Componentes en Carrito: {:#?}", ctx.accounts.carrito.componentes);
        Ok(())
    }
}

// --- ESTADO DEL PROGRAMA ---

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Componente {
    #[max_len(20)]
    pub categoria: String,
    #[max_len(30)]
    pub modelo: String,
    pub precio_estimado: u32,
}

#[account]
#[derive(InitSpace)]
pub struct CarritoPC {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_proyecto: String,
    #[max_len(12)] // Capacidad para 12 componentes (Suficiente para una build completa)
    pub componentes: Vec<Componente>,
}

// --- CONTEXTOS ---

#[derive(Accounts)]
pub struct CrearCarrito<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + CarritoPC::INIT_SPACE,
        seeds = [b"pcbuild", owner.key().as_ref()],
        bump
    )]
    pub carrito: Account<'info, CarritoPC>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarCarrito<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub carrito: Account<'info, CarritoPC>,
}

// --- ERRORES ---

#[error_code]
pub enum Errores {
    #[msg("Acceso denegado: No eres el dueño de este carrito.")]
    NoEresElDueno,
    #[msg("El componente no se encuentra en el carrito.")]
    ComponenteNoEncontrado,
}
