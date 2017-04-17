/// Intrumentation Trace Macrocell (aka printf debug port)
/*
#define SCS_BASE            (0xE000E000UL)                            /*!< System Control Space Base Address  */

#define ITM_BASE            (0xE0000000UL)                            /*!< ITM Base Address                   */

#define CoreDebug_BASE      (0xE000EDF0UL)                            /*!< Core Debug Base Address            */

#define SysTick_BASE        (SCS_BASE +  0x0010UL)                    /*!< SysTick Base Address               */

#define NVIC_BASE           (SCS_BASE +  0x0100UL)                    /*!< NVIC Base Address                  */

#define SCB_BASE            (SCS_BASE +  0x0D00UL)                    /*!< System Control Block Base Address  */



#define SCnSCB              ((SCnSCB_Type    *)     SCS_BASE      )   /*!< System control Register not in SCB */

#define SCB                 ((SCB_Type       *)     SCB_BASE      )   /*!< SCB configuration struct           */

#define SysTick             ((SysTick_Type   *)     SysTick_BASE  )   /*!< SysTick configuration struct       */

#define NVIC                ((NVIC_Type      *)     NVIC_BASE     )   /*!< NVIC configuration struct          */

#define ITM                 ((ITM_Type       *)     ITM_BASE      )   /*!< ITM configuration struct           */

#define CoreDebug           ((CoreDebug_Type *)     CoreDebug_BASE)   /*!< Core Debug configuration struct    */
*/

pub struct InstrumentationTraceMacrocell {

}
