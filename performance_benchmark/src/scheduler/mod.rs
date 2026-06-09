use crossbeam_channel::Sender;

/// O `pub` aqui torna a struct visível para o resto do projeto.
pub struct Scheduler;

impl Scheduler {
    /// Despacha o pacote para o canal de alta ou baixa prioridade.
    /// O `pub` é necessário para que a função seja chamável de fora deste módulo.
    pub fn dispatch<T>(priority: u8, high_tx: &Sender<T>, low_tx: &Sender<T>, pkt: T) {
        if priority == 0 {
            // Tenta enviar para o canal de alta prioridade
            let _ = high_tx.try_send(pkt);
        } else {
            // Tenta enviar para o canal de baixa prioridade
            let _ = low_tx.try_send(pkt);
        }
    }
}