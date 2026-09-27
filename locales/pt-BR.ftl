# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Portuguese (Brazil).
#
# Every message here must also exist in every other locale file;
# `cargo test -p usbnexus-i18n` enforces this.

## Command line help

app-about = USB sobre IP seguro: compartilhe dispositivos USB pela rede com criptografia, pareamento e reconexão automática.
arg-lang = Idioma da interface (ex.: en, tr). Padrão: idioma do sistema.
arg-state-dir = Diretório para as chaves e os computadores pareados.
arg-name = Nome deste computador, como aparece para os outros.
arg-verbose = Mostrar mensagens detalhadas de log.

cmd-daemon-about = Executar o serviço USB Nexus (usado pelo aplicativo de desktop).
arg-allow-all-users = Permitir que qualquer usuário local controle o serviço (padrão: membros do grupo usbnexus).
arg-socket = Caminho do socket de controle local do serviço.
cmd-service-about = Instalar ou remover o serviço USB Nexus do Windows.
cmd-install-about = Instalar o serviço, iniciá-lo agora e em toda inicialização (execute como administrador).
cmd-uninstall-about = Parar e remover o serviço.
cmd-serve-about = Compartilhar os dispositivos USB deste computador.
arg-export = Bus ID de um dispositivo a compartilhar (pode repetir). Veja: usbnexus local
arg-listen = Endereço e porta em que ouvir.
arg-pair = Abrir uma janela de pareamento ao iniciar e mostrar o PIN.
arg-no-mdns = Não anunciar este servidor na rede local.

cmd-pin-about = Mostrar um PIN para parear um novo computador com o servidor em execução.
arg-seconds = Por quanto tempo o PIN permanece válido, em segundos.

cmd-local-about = Listar os dispositivos USB conectados a este computador.
cmd-discover-about = Encontrar servidores USB Nexus na rede local.
arg-timeout = Por quanto tempo procurar, em segundos.
cmd-pair-about = Parear com um servidor usando o PIN mostrado por ele.
arg-server = Servidor: nome pareado, fingerprint ou host[:porta].
arg-pin = PIN mostrado pelo servidor (solicitado se não informado).
cmd-list-about = Listar os dispositivos compartilhados por um servidor.
cmd-allow-user-about = Permitir que um usuário controle o serviço pelo aplicativo de desktop (adiciona-o ao grupo usbnexus).
arg-user = Nome de usuário.
allow-user-invalid = “{ $user }” não é um nome de usuário válido.
allow-user-failed = Não foi possível adicionar “{ $user }” ao grupo usbnexus.
allow-user-done = “{ $user }” agora pode controlar o serviço USB Nexus.
allow-user-relogin = Isso passa a valer depois que o usuário entrar novamente.
cmd-attach-about = Usar um dispositivo remoto neste computador; reconecta automaticamente.
arg-busid = Bus ID do dispositivo remoto.
cmd-peers-about = Listar os computadores pareados.
cmd-forget-about = Remover um computador pareado.
arg-peer = Nome ou fingerprint do computador pareado.

## General

error-prefix = Erro: { $detail }
hint-root = Esta operação exige privilégios de administrador. Tente novamente com sudo.
unsupported-os = Este comando ainda não é compatível com este sistema operacional.
yes = sim
no = não

service-installed = O serviço USB Nexus foi instalado e iniciado.
service-removed = O serviço USB Nexus foi removido.

## Server

serve-started = O servidor “{ $name }” está escutando em { $addr }.
serve-fingerprint = Fingerprint: { $fp }
serve-exporting = Dispositivos compartilhados:
serve-no-exports = Nenhum dispositivo está compartilhado. Adicione --export BUSID (liste os dispositivos com: usbnexus local).
serve-pairing-pin = PIN de pareamento: { $pin } (válido por { $seconds } segundos)
serve-paired = Pareado com “{ $name }”.
serve-pairing-failed = Tentativa de pareamento sem sucesso a partir de { $addr }.
serve-exported = { $busid } agora está em uso por “{ $client }”.
serve-released = { $busid } foi liberado por “{ $client }”.
serve-stopping = Parando; devolvendo os dispositivos aos drivers normais…
serve-mdns-failed = A descoberta na rede local não está disponível: { $detail }
serve-bind-failed = Não foi possível preparar { $busid } para compartilhamento: { $detail }

## Pairing

pin-show = PIN de pareamento: { $pin }
pin-hint = No outro computador, execute “usbnexus pair { $name }” em até { $seconds } segundos.
pin-no-server = Nenhum servidor USB Nexus em execução foi encontrado. Inicie um com: usbnexus serve
pair-enter-pin = Digite o PIN mostrado no servidor:
pair-ok = Pareado com “{ $name }” ({ $fp }).
pair-already = Já pareado com “{ $name }”.

## Devices

local-header = Dispositivos USB neste computador:
local-empty = Nenhum dispositivo USB encontrado.
list-header = Dispositivos compartilhados por “{ $name }”:
list-empty = Este servidor não compartilha nenhum dispositivo.
list-in-use = em uso
col-busid = BUS ID
col-id = VID:PID
col-speed = VELOCIDADE
col-product = PRODUTO
col-driver = DRIVER
col-state = ESTADO
col-name = NOME
col-fingerprint = FINGERPRINT
col-address = ENDEREÇO

## Discovery and peers

discover-searching = Procurando na rede local…
discover-none = Nenhum servidor USB Nexus foi encontrado.
discover-paired = pareado
peers-empty = Ainda não há computadores pareados.
forget-ok = “{ $name }” foi removido.
forget-unknown = Nenhum computador pareado corresponde a “{ $peer }”.

## Attaching

attach-connecting = Conectando a { $addr }…
attach-attached = { $busid } está conectado (porta virtual { $port }).
attach-disconnected = Conexão perdida: { $reason }
attach-retrying = Reconectando em { $seconds } segundos…
attach-detached = O dispositivo foi desconectado.
attach-stop-hint = Pressione Ctrl+C para desconectar.

## Errors

err-pairing-required = “{ $name }” ainda não está pareado com este computador. Execute: usbnexus pair { $target }
err-not-found = O servidor não foi encontrado na rede.
err-version = O servidor está executando uma versão incompatível do USB Nexus.
err-not-trusted = Este computador não está pareado com o servidor.
err-pairing-closed = O pareamento não está aberto no servidor. No servidor, execute: usbnexus pin
err-pairing-failed = O pareamento falhou. Verifique o PIN e tente novamente.
err-no-such-device = O servidor não compartilha este dispositivo.
err-device-busy = O dispositivo está em uso por outro computador.
err-internal = O servidor relatou um erro interno.
err-protocol = Resposta inesperada do servidor.
err-connection-lost = A conexão com o computador foi perdida.
err-unsupported = Isso ainda não está disponível neste sistema operacional.
err-driver-missing = O driver usbip-win2 não está instalado. Execute a instalação do USB Nexus novamente e aceite instalar o usbip-win2.
err-driver-outdated = O driver usbip-win2 instalado está muito antigo. Execute a instalação do USB Nexus novamente e aceite atualizar o usbip-win2.
err-vboxusb-missing = Os drivers USB do VirtualBox necessários para o compartilhamento não estão instalados. Reinstale o USB Nexus.
err-device-in-use-by-os = O sistema operacional está usando este dispositivo com seu próprio driver, portanto ele não pode ser compartilhado a partir deste computador.
err-unreachable = Não foi possível alcançar o computador. Verifique se ele está ligado e conectado à rede.
err-cancelled = A ação foi cancelada.
err-permission-denied = O USB Nexus não tem as permissões necessárias.
err-invalid = A solicitação não foi compreendida.
err-other = Algo deu errado: { $detail }

## Help layout

help-usage = Uso:
help-arguments = Argumentos
help-options = Opções
help-commands = Comandos
arg-help = Mostrar ajuda.
arg-version = Mostrar a versão.

## Peers

peers-servers = Servidores pareados (este computador pode usar os dispositivos deles):
peers-clients = Clientes pareados (podem usar os dispositivos deste computador):

## Desktop app

gui-nav-this-computer = Este computador
gui-nav-network = Computadores na rede
gui-nav-connected = Dispositivos conectados
gui-nav-paired = Computadores pareados
gui-language = Idioma
gui-this-title = Dispositivos deste computador
gui-this-subtitle = Escolha quais dispositivos outros computadores podem usar.
gui-share = Compartilhar
gui-shared = Compartilhado
gui-not-shared = Não compartilhado
gui-used-by = Em uso por { $name }
gui-no-local-devices = Nenhum dispositivo USB foi encontrado neste computador.
gui-unnamed-device = Dispositivo USB
gui-pair-new = Parear um novo computador
gui-pin-title = PIN de pareamento
gui-pin-body = Digite este PIN no outro computador.
gui-pin-remaining = Válido por mais { $seconds } segundos
gui-pin-expired = O PIN expirou.
gui-pin-new = Novo PIN
gui-pin-stop = Parar pareamento
gui-close = Fechar
gui-cancel = Cancelar
gui-network-title = Computadores na rede
gui-network-subtitle = Computadores USB Nexus encontrados na sua rede local.
gui-refresh = Atualizar
gui-searching = Procurando na rede…
gui-none-found = Nenhum computador foi encontrado. Verifique se o USB Nexus está em execução no outro computador ou adicione-o pelo endereço.
gui-add-by-address = Adicionar pelo endereço
gui-address = Endereço
gui-address-hint = ex.: 192.168.1.20
gui-pair = Parear
gui-paired = Pareado
gui-pair-title = Parear com { $name }
gui-pair-body = Em { $name }, escolha “Parear um novo computador” e digite o PIN mostrado lá.
gui-pin = PIN
gui-pair-done = Pareado com { $name }.
gui-devices-of = Dispositivos compartilhados por { $name }
gui-no-remote-devices = Este computador não compartilha nenhum dispositivo.
gui-connect = Conectar
gui-disconnect = Desconectar
gui-in-use-elsewhere = Em uso por outro computador
gui-connected-here = Conectado a este computador
gui-back = Voltar
gui-connected-title = Dispositivos conectados a este computador
gui-connected-subtitle = Os dispositivos remotos permanecem conectados e se reconectam automaticamente se a rede cair.
gui-connected-empty = Nenhum dispositivo remoto está conectado. Abra “Rede” para conectar um.
gui-on-computer = em { $name }
gui-state-connecting = Conectando…
gui-state-attached = Conectado
gui-state-retrying = Reconectando em { $seconds } s
gui-state-stopped = Desconectado
gui-state-failed = Falhou
gui-reconnect = Reconectar
gui-remove = Remover
gui-paired-title = Computadores pareados
gui-paired-subtitle = Os computadores são pareados uma vez com um PIN e reconhecidos automaticamente depois disso.
gui-paired-servers = Computadores cujos dispositivos você pode usar
gui-paired-clients = Computadores que podem usar os seus dispositivos
gui-paired-empty = Nenhum ainda.
gui-remove-confirm = Remover { $name }? Você precisará parear novamente para usá-lo.
gui-fingerprint = Fingerprint
gui-service-denied-title = Este usuário não pode controlar o serviço USB Nexus
gui-service-denied-body = Somente administradores e membros do grupo usbnexus podem. Permita este usuário (será solicitada a senha de administrador) ou execute como administrador:
gui-service-denied-command = Comando:
gui-service-denied-user = USUÁRIO
gui-grant-access = Permitir este usuário
gui-grant-access-done = Acesso concedido.
gui-setup-kernel-modules = Faltam módulos do kernel necessários para USB sobre IP: { $modules }. Instale-os; o serviço os carrega por conta própria:
gui-setup-kernel-modules-nocmd = Eles vêm com o kernel da maioria das distribuições (Ubuntu: linux-modules-extra, Fedora: kernel-modules-extra).
gui-service-down-title = O serviço USB Nexus não está em execução
gui-service-down-body = Inicie o serviço; esta janela se conecta a ele automaticamente.
gui-service-down-linux = No Linux, execute:
gui-service-down-windows = No Windows, como administrador, execute:
gui-service-down-macos = No macOS, execute:
gui-retry = Tentar novamente
gui-details = Detalhes
err-service-unavailable = Não foi possível alcançar o serviço USB Nexus.

## Web interface

gui-web-login-title = Entrar em { $name }
gui-web-password = Senha
gui-web-sign-in = Entrar
gui-web-sign-out = Sair
gui-web-wrong-password = Senha incorreta.
gui-web-locked = Muitas tentativas. Tente novamente em { $seconds } segundos.
cmd-web-about = Ativar ou desativar a interface web.
cmd-enable-about = Ativar a interface web (pede uma senha na primeira vez).
cmd-disable-about = Desativar a interface web.
cmd-password-about = Alterar a senha da interface web.
cmd-status-about = Mostrar se a interface web está ativada e onde.
arg-lan = Permitir acesso de outros computadores da rede.
arg-port = Porta TCP da interface web.
web-on = A interface web está ativada:
web-off = A interface web está desativada.
web-local-only = Somente este computador pode abri-la. Use --lan para permitir outros computadores.
web-fingerprint = O navegador vai avisar sobre o certificado; seu fingerprint deve ser: { $fp }
web-trusted = Os navegadores deste computador confiam no certificado; outros computadores vão avisar. O fingerprint dele é: { $fp }
web-not-running = A interface web está ativada, mas não foi possível iniciá-la: { $detail }
web-password-prompt = Nova senha da interface web:
web-password-repeat = Repita a senha:
web-password-mismatch = As senhas não coincidem.
web-password-set = A senha da interface web foi alterada.
err-weak-password = A senha deve ter pelo menos 8 caracteres.
err-port-in-use = Esta porta está sendo usada por outro programa. Escolha outra porta.
err-password-required = Defina primeiro uma senha para a interface web: usbnexus web password
err-forbidden = Isso só pode ser alterado no próprio computador.
err-not-logged-in = Entre novamente, por favor.

## Hotplug, access control and usage log

arg-device = Dispositivo: identidade ou bus id (veja: usbnexus list SERVIDOR).
cmd-policy-about = Mostrar ou escolher quem pode usar os dispositivos compartilhados.
arg-policy = open: todo computador pareado pode usar todo dispositivo compartilhado; restricted: somente os computadores permitidos por dispositivo.
arg-no-server = Não configurar o compartilhamento dos dispositivos USB deste computador.
arg-no-client = Não configurar o uso de dispositivos USB de outros computadores.
arg-web = Interface web: off (desativada), local (somente este computador) ou network (toda a rede).
arg-web-port = Porta da interface web (padrão 3242).
arg-web-password-file = Arquivo com a nova senha da interface web.
cmd-history-about = Mostrar o histórico de uso dos dispositivos compartilhados deste computador.
arg-csv = Exibir todas as entradas em CSV (ex.: para salvar em um arquivo).
arg-limit = Número de entradas a mostrar.
col-device-id = ID DO DISPOSITIVO
col-time = HORA (UTC)
col-event = EVENTO
col-computer = COMPUTADOR
col-duration = DURAÇÃO
state-unplugged = não conectado
state-no-permission = sem permissão
serve-denied = “{ $client }” não tem permissão para usar { $busid }.
attach-waiting-device = O dispositivo não está conectado ao servidor; aguardando…
attach-queued = Outro computador está usando o dispositivo; este é o número { $position } na fila.
policy-open = Todo computador pareado pode usar todo dispositivo compartilhado (aberta).
policy-restricted = Os computadores pareados só podem usar os dispositivos que têm permissão para usar (restrita).
history-header = Histórico de uso (as entradas são mantidas por { $days } dias):
history-empty = O histórico de uso está vazio.
history-paired = pareado
history-pairing-failed = PIN incorreto
history-attached = começou a usar
history-detached = parou de usar
history-denied = recusado (sem permissão)
err-access-denied = Este computador não tem permissão para usar o dispositivo.

gui-nav-history = Histórico
gui-nav-settings = Configurações
gui-not-plugged-in = Não conectado
gui-tracked-by-port = rastreado pela porta
gui-tracked-by-port-hint = Este dispositivo não tem número de série, então é reconhecido pela porta USB em que está conectado. Conecte-o na mesma porta novamente.
gui-no-permission = Sem permissão
gui-no-permission-hint = O proprietário daquele computador não permitiu que este computador use o dispositivo.
gui-connect-when-plugged-in = Conecta automaticamente assim que o dispositivo for plugado.
gui-state-waiting-device = Aguardando o dispositivo
gui-state-queued = Em uso em outro lugar; número { $position } na fila
gui-save = Salvar
gui-saved = Salvo.
gui-skip = Pular
gui-access-title = Quem pode usar este dispositivo
gui-access-everyone = Todos os computadores pareados
gui-access-some = Computadores selecionados ({ $count })
gui-access-nobody = Nenhum computador ainda
gui-access-mode-default = Seguir a configuração padrão
gui-access-default-open = Atualmente: todos os computadores pareados.
gui-access-default-restricted = Atualmente: somente os computadores selecionados abaixo.
gui-access-mode-open = Todos os computadores pareados
gui-access-mode-open-body = Todo computador pareado com este pode usá-lo.
gui-access-mode-selected = Somente computadores selecionados
gui-access-mode-selected-body = Somente os computadores marcados abaixo podem usá-lo.
gui-access-computers = Computadores com permissão para usá-lo
gui-access-revoke-note = Um computador que perde a permissão é desconectado do dispositivo imediatamente.
gui-client-devices = Dispositivos
gui-client-devices-title = Dispositivos que { $name } pode usar
gui-client-devices-body = Marque os dispositivos compartilhados que este computador pode usar.
gui-client-devices-after-pairing = Somente computadores autorizados podem usar dispositivos compartilhados. Marque os dispositivos que este computador pode usar; se você pular esta etapa, ele não poderá usar nenhum por enquanto.
gui-no-shared-devices = Este computador ainda não compartilha nenhum dispositivo.
gui-roles-title = Como este computador é usado
gui-roles-body = As telas de um uso não escolhido ficam ocultas. Adicionar um uso instala o que for necessário.
gui-role-server = Usar como servidor (compartilhar os dispositivos USB deste computador)
gui-role-client = Usar como cliente (usar dispositivos USB de outros computadores)
gui-roles-client-note = Se necessário, o driver usbip-win2 é instalado; os dispositivos USB param por alguns segundos e pode ser preciso reiniciar o Windows.
gui-roles-applying = Aplicando…
gui-reboot-required = Reinicie o computador para concluir a configuração.
gui-used-by-waiting = { $name } está usando · { $count } aguardando
gui-col-permissions = Permissões
gui-col-status = Status
gui-in-use-title = Em uso por
gui-nobody-using = Ninguém está usando o dispositivo agora.
gui-since = desde { $time }
gui-disconnect-user = Desconectar
gui-disconnected-note = { $name } foi desconectado. Se continuar solicitando o dispositivo, ele se reconecta em alguns segundos; para bloqueá-lo definitivamente, desmarque-o na lista à esquerda e salve.
gui-queue-title = Na fila ({ $count })
gui-queue-empty = Ninguém está esperando.
gui-badge-using = usando
gui-badge-queued = na fila ({ $position })
gui-handover-title = Transferência automática
gui-handover-default-on = Padrão (ativada, { $seconds } s)
gui-handover-default-off = Padrão (desativada)
gui-handover-on = Ativada
gui-handover-off = Desativada
gui-handover-before = Enquanto outro computador espera, depois de
gui-handover-after = segundos sem uso, ele passa para o próximo.
gui-kind-storage = Armazenamento
gui-kind-input = Teclado / mouse
gui-kind-printer = Impressora
gui-kind-dongle = Dongle de licença
gui-kind-other = Outro dispositivo
gui-web-title = Interface web
gui-web-body = Gerencie este computador a partir de um navegador, com senha.
gui-web-confirm-off = Desativar a interface web? Esta página deixará de funcionar. Ela pode ser ativada novamente no aplicativo de desktop ou com “usbnexus web enable” no próprio computador.
gui-web-confirm-local = Permitir acesso somente deste computador? Esta página foi aberta pela rede e deixará de funcionar.
gui-web-turned-off = A interface web está desativada. Ela pode ser ativada novamente no aplicativo de desktop ou com “usbnexus web enable” no próprio computador.
gui-web-enabled = Interface web ativada
gui-web-access-local = Somente este computador
gui-web-access-network = Toda a rede
gui-web-port = Porta
gui-web-new-password = Nova senha
gui-web-repeat-password = Nova senha (novamente)
gui-web-password-keep = Pelo menos 8 caracteres. Deixe em branco para manter a senha atual.
gui-web-password-required = Pelo menos 8 caracteres.
gui-web-mismatch = As senhas não coincidem.
gui-web-open-at = Abrir em:
gui-web-fingerprint = O navegador avisa sobre o certificado; o fingerprint dele é { $fp }.
gui-web-trusted = Os navegadores deste computador confiam no certificado; outros computadores avisam sobre ele. O fingerprint dele é { $fp }.
gui-policy-title = Quem pode usar os dispositivos compartilhados
gui-policy-body = Os computadores sempre precisam ser pareados primeiro. Este é o padrão para todo dispositivo compartilhado; cada dispositivo pode substituí-lo.
gui-policy-first-title = Quem pode usar os seus dispositivos compartilhados?
gui-policy-first-body = Os computadores sempre precisam ser pareados primeiro com um PIN. Escolha o que os computadores pareados podem fazer:
gui-policy-later = Você pode alterar isso mais tarde em Configurações.
gui-policy-open = Todo computador pareado
gui-policy-open-body = Todo computador pareado pode usar todo dispositivo compartilhado.
gui-policy-restricted = Somente computadores autorizados
gui-policy-restricted-body = Você escolhe, por dispositivo, quais computadores pareados podem usá-lo.
gui-retention-title = Histórico de uso
gui-retention-body = Pareamentos, PINs incorretos, uso de dispositivos e solicitações recusadas são registrados. As entradas mais antigas são excluídas automaticamente.
gui-retention-days = Manter entradas por (dias)
gui-settings-title = Configurações
gui-startup-title = Inicialização
gui-startup-body = Fechar a janela mantém o USB Nexus na área de notificação; o serviço continua compartilhando e mantendo as conexões de qualquer forma.
gui-startup-enabled = Iniciar automaticamente ao entrar na sessão
gui-tray-open = Abrir o USB Nexus
gui-tray-quit = Sair
gui-history-title = Histórico
gui-history-subtitle = Quem usou os dispositivos deste computador, e quando. As entradas são mantidas por { $days } dias.
gui-history-empty = Nada foi registrado ainda.
gui-history-export = Exportar CSV
gui-history-time = Hora
gui-history-event = Evento
gui-history-computer = Computador
gui-history-device = Dispositivo
gui-history-device-id = ID do dispositivo
gui-history-duration = Duração
gui-history-paired = Pareado
gui-history-pairing-failed = PIN incorreto
gui-history-attached = Começou a usar
gui-history-detached = Parou de usar
gui-history-denied = Recusado: sem permissão

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = Como você vai usar o USB Nexus?
setup-roles-subtitle = Escolha o que este computador vai fazer.
setup-role-server = Usar como servidor
setup-role-client = Usar como cliente
setup-role-web = Acesso pela web
setup-usbip-install-note = O driver usbip-win2 também será instalado. Os dispositivos USB param por alguns segundos durante a instalação, e o Windows precisará ser reiniciado depois.
setup-usbip-update-note = O driver usbip-win2 instalado está muito antigo e será atualizado. Os dispositivos USB param por alguns segundos durante a instalação, e o Windows precisará ser reiniciado depois.
setup-usbip-present-note = O driver usbip-win2 já está instalado neste computador.
setup-usbip-failed = Não foi possível instalar o driver usbip-win2. Você pode executar a instalação do USB Nexus novamente mais tarde para tentar de novo.
setup-service-failed = Não foi possível configurar o serviço USB Nexus. Os detalhes estão no log de instalação.
setup-web-title = Interface web
setup-web-subtitle = Configurações para gerenciar este computador a partir de um navegador.
setup-web-access = Acesso:
setup-web-local = Somente este computador
setup-web-network = Toda a rede
setup-web-port = Porta:
setup-web-port-free = ✓ A porta está disponível
setup-web-port-busy = ✗ Esta porta está sendo usada por outro programa
setup-web-port-invalid = ✗ Digite um número entre 1 e 65535
setup-web-password = Senha:
setup-web-password-repeat = Senha (novamente):
setup-web-password-hint = Pelo menos 8 caracteres.
setup-web-password-keep = Pelo menos 8 caracteres. Deixe em branco para manter a senha atual.
